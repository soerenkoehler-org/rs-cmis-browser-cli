use std::io::Result;

use crate::{
    cmis::{
        Object, ObjectId, QueryBuilder,
        def::{
            CMIS_DIR_ATTRIBUTES, CMIS_OBJECT_ID, CMIS_SELECTOR, CMIS_SELECTOR_CHILDREN,
            CMIS_SUCCINCT, ChildrenResponse, PageReader,
        },
        get_name,
        props::{get_object_id, get_parent_id},
        query::ops::{and, id, in_folder, name},
    },
    config::GetChildrenMethod::{Query, Selector},
    http::read_object,
    repl::Session,
};

impl ObjectId {
    pub fn is_empty(&self) -> bool {
        self.id.is_empty()
    }

    pub fn from_path(path: &str, session: &Session) -> Result<ObjectId> {
        let base = if path.starts_with('/') || session.cwd.is_empty() {
            Self::new(&session.repository.root_folder_id)
        } else {
            session.cwd.clone()
        };
        base.select_path_recursive(path, session)
    }

    // TODO pub fn from_doc(doc_id: &str, session: &Session) -> Result<String> {}

    pub fn from_id(id: &str) -> Result<ObjectId> {
        Ok(Self::new(id))
    }

    fn new(id: &str) -> ObjectId {
        Self { id: id.to_string() }
    }

    fn select_path_recursive(self, path: &str, session: &Session) -> Result<ObjectId> {
        let mut parts = path.splitn(2, "/");

        // process first path element
        let next_id = match parts.next() {
            None => self,
            Some("") => self,
            Some(".") => self,
            Some("..") => self.get_parent(session)?,
            Some(child_name) => self.get_child(child_name, session)?,
        };

        // recurse, if path has more elements
        match parts.next() {
            Some(path) => next_id.select_path_recursive(path, session),
            None => Ok(next_id),
        }
    }

    fn get_parent(&self, session: &Session) -> Result<ObjectId> {
        let this_id = &self.id;
        let objects = QueryBuilder::from_folders()
            .select(CMIS_DIR_ATTRIBUTES)
            .condition(&id(this_id))
            .read_paged(session)?;
        match objects.len() {
            1 => Self::from_id(get_parent_id(&objects[0].succinct_properties)?),
            0 => io_error!(format!("not found: {this_id}")),
            _ => io_error!(format!("bad tree: {this_id} is not unique")),
        }
    }

    fn get_child(&self, child_name: &str, session: &Session) -> Result<ObjectId> {
        match session.config.global.get_children_method {
            Selector => self.get_child_by_selector(child_name, session),
            Query => self.get_child_by_query(child_name, session),
        }
    }

    fn get_child_by_selector(&self, child_name: &str, session: &Session) -> Result<ObjectId> {
        // FIXME as macro
        let Some(connection) = session.config.connections.get(&session.connection_name) else {
            return io_error!(format!(
                "No config for connection {}",
                session.connection_name
            ));
        };
        let request = session
            .http_session
            .get_request(connection, &session.repository.root_folder_url)?;
        let request = request
            .query(CMIS_SUCCINCT, "true")
            .query(CMIS_SELECTOR, CMIS_SELECTOR_CHILDREN)
            .query(CMIS_OBJECT_ID, &self.id);
        let children: ChildrenResponse = read_object(request)?;
        match children.objects.iter().find(|child| {
            matches!(
                get_name(&child.object.succinct_properties),
                Ok(name) if name == child_name
            )
        }) {
            Some(child) => {
                let child_id = get_object_id(&child.object.succinct_properties)?;
                Self::from_id(child_id)
            }
            _ => io_error!(format!("not found: {child_name}")),
        }
    }

    fn get_child_by_query(&self, child_name: &str, session: &Session) -> Result<ObjectId> {
        let objects = QueryBuilder::from_objects()
            .select(CMIS_DIR_ATTRIBUTES)
            .condition(&and(&in_folder(&self.id), &name(child_name)))
            .read_paged(session)?;
        match objects.len() {
            1 => Self::from_id(get_object_id(&objects[0].succinct_properties)?),
            0 => io_error!(format!("not found: {child_name}")),
            _ => io_error!(format!("bad tree: {child_name} is not unique")),
        }
    }

    pub fn get_children(&self, session: &Session) -> Result<Vec<Object>> {
        match session.config.global.get_children_method {
            Selector => PageReader::new()
                .read_paged(
                    session,
                    |request| {
                        request
                            .query(CMIS_SELECTOR, CMIS_SELECTOR_CHILDREN)
                            .query(CMIS_OBJECT_ID, &self.id)
                    },
                    |response: ChildrenResponse| response.objects,
                )
                .collect(|child| child.object),
            Query => QueryBuilder::from_objects()
                .select(CMIS_DIR_ATTRIBUTES)
                .condition(&in_folder(&self.id))
                .read_paged(session),
        }
    }
}
