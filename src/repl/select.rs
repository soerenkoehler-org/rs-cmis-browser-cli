use std::io::Result;

use crate::{
    cmis::QuerySet,
    repl::def::{
        Repl,
        ReplState::{self, Continue},
    },
};

impl Repl {
    pub(super) fn command_select(&mut self, query: &str) -> Result<ReplState> {
        let session = get_session!(self);
        let data = QuerySet::new().add(query.to_string()).call(session)?;
        let pretty = serde_json::to_string_pretty(&data)?;
        Ok(Continue(vec![format!("{}", pretty)]))
    }
}
