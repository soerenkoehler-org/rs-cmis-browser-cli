use std::{io::Result, iter::once, time::Instant};

use serde_json::Value;

use crate::{
    cmis::{
        CMIS_TYPE_ID_DOCUMENT, CMIS_TYPE_ID_FOLDER, Object, ObjectId, get_length, get_name,
        get_object_type,
    },
    repl::def::{
        Repl,
        ReplState::{self, Continue},
    },
    util::duration_to_string,
};

impl Repl {
    pub(super) fn command_cd(&mut self, path: &str) -> Result<ReplState> {
        let session = get_session!(self);
        session.cwd = ObjectId::from_path(path, session)?;
        Ok(Continue(vec![]))
    }

    // FIXME unused
    // pub(super) fn command_dir(&mut self, path: &str) -> Result<ReplState> {
    //     let session = get_session!(self);

    //     let start_time = Instant::now();
    //     let parent = ObjectId::from_path(path, session)?;
    //     let children = parent.get_children(session)?;

    //     let mut entries = children.iter().map(create_dir_entry).collect::<Vec<_>>();
    //     let length_column_width = entries
    //         .iter()
    //         .map(|(_, length, _)| length.len())
    //         .max()
    //         .unwrap_or(1);
    //     entries.sort_by_key(|(_, _, name)| *name);

    //     let entries_count = entries.len();
    //     let result = entries
    //         .iter()
    //         .map(|(type_id, length, name)| {
    //             format!(
    //                 "{type_id} \u{205e} {length: >width$} \u{205e} {name}",
    //                 width = length_column_width
    //             )
    //         })
    //         .chain(once(format!(
    //             "{} items ({})",
    //             entries_count,
    //             duration_to_string(&start_time.elapsed())
    //         )))
    //         .collect::<Vec<_>>();
    //     Ok(ReplState::Continue(result))
    // }

    pub(super) fn command_stat(&mut self, path: &str) -> Result<ReplState> {
        let session = get_session!(self);
        let object = ObjectId::from_path(path, session)?;
        let json = object.get_raw_properties(session)?;
        let properties = serde_json::from_str::<Value>(&json)?;
        let pretty = serde_json::to_string_pretty(&properties)?;
        Ok(ReplState::Continue(vec![pretty]))
    }
}

fn create_dir_entry(wrapped_object: &Object) -> (&str, String, &str) {
    let properties = &wrapped_object.succinct_properties;

    let type_id = get_object_type(properties).unwrap_or_default();

    (
        // entry type
        match type_id {
            CMIS_TYPE_ID_FOLDER => "dir",
            CMIS_TYPE_ID_DOCUMENT => "\u{2027}\u{2027}\u{2027}",
            _ => "\u{2027}?\u{2027}",
        },
        // entry length as string
        match type_id {
            CMIS_TYPE_ID_DOCUMENT => match get_length(properties) {
                Ok(length) => format!("{length}"),
                _ => "?".to_string(),
            },
            _ => "".to_string(),
        },
        // entry name
        match get_name(properties) {
            Ok(name) => name,
            _ => "?",
        },
    )
}
