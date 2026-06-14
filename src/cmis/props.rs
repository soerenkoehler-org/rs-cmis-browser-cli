use std::io::Result;

use serde_json::{Number, Value};
use ureq::{RequestBuilder, typestate::WithoutBody};

use crate::{
    cmis::{
        ObjectId,
        def::{
            CMIS_DIR_ATTRIBUTES, CMIS_FILTER, CMIS_OBJECT_ID, CMIS_SELECTOR,
            CMIS_SELECTOR_PROPERTIES, CMIS_SUCCINCT, PropertyList,
        },
    },
    http::{read_object, read_string},
    repl::Session,
};

macro_rules! get_property {
    ($prop:expr, $name:expr, $val:ident, $null:expr) => {
        match $prop.get($name) {
            Some(Value::$val(property)) => Ok(property),
            Some(Value::Null) => Ok($null),
            _ => io_error!(format!("property not found: {}", $name)),
        }
    };
}

macro_rules! get_u64 {
    ($f:ident, $n:expr) => {
        pub fn $f(properties: &PropertyList) -> Result<u64> {
            let default = Number::from(0);
            let number = get_property!(properties, $n, Number, &default)?;
            match number.as_u64() {
                Some(number) => Ok(number),
                None => Ok(0),
            }
        }
    };
}

macro_rules! get_str {
    ($f:ident, $n:expr) => {
        pub fn $f(properties: &PropertyList) -> Result<&str> {
            get_property!(properties, $n, String, "")
        }
    };
}

get_str!(get_object_id, "cmis:objectId");
get_str!(get_object_type, "cmis:objectTypeId");
get_str!(get_name, "cmis:name");
get_str!(get_cmis_path, "cmis:path");
get_str!(get_path_tokens, "path_tokens");
get_str!(get_parent_id, "cmis:parentId");
get_u64!(get_length, "cmis:contentStreamLength");

pub fn get_path(properties: &PropertyList) -> Result<&str> {
    match (get_cmis_path(properties), get_path_tokens(properties)) {
        (Ok(path), _) => Ok(path),
        (_, Ok(path)) => Ok(path),
        _ => io_error!("missing path attribute: cmis:path or path_tokens"),
    }
}

#[allow(dead_code)]
pub fn get_properties(object_id: &str, session: &Session) -> Result<PropertyList> {
    let request = create_property_request(object_id, session)?;
    read_object(request)
}

impl ObjectId {
    pub fn get_raw_properties(&self, session: &Session) -> Result<String> {
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
            .query(CMIS_SELECTOR, CMIS_SELECTOR_PROPERTIES)
            .query(CMIS_OBJECT_ID, &self.id)
            .query(CMIS_FILTER, CMIS_DIR_ATTRIBUTES);
        read_string(request)
    }

    pub fn get_dir_properties(&self, session: &Session) -> Result<PropertyList> {
        let request = create_property_request(&self.id, session)?;
        read_object(request.query(CMIS_FILTER, CMIS_DIR_ATTRIBUTES))
    }
}

fn create_property_request(
    object_id: &str,
    session: &Session,
) -> Result<RequestBuilder<WithoutBody>> {
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
        .query(CMIS_SELECTOR, CMIS_SELECTOR_PROPERTIES)
        .query(CMIS_OBJECT_ID, object_id);
    Ok(request)
}
