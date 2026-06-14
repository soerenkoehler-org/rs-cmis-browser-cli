use std::io::Result;

use crate::{cmis::def::RepositoryList, repl::Session};

pub fn get_repository_list(session: &Session) -> Result<RepositoryList> {
    // FIXME make macro
    let Some(connection) = session.config.connections.get(&session.connection_name) else {
        return io_error!(format!(
            "No config for connection {}",
            session.connection_name
        ));
    };
    match session
        .http_session
        .get_request(&connection, &connection.url)?
        .call()
    {
        Ok(mut response) => match response.body_mut().read_to_string() {
            Ok(body) => match serde_json::from_str::<RepositoryList>(&body) {
                Ok(repository_list) => Ok(repository_list),
                Err(err) => io_error!(err),
            },
            Err(err) => io_error!(err),
        },
        Err(err) => io_error!(err),
    }
}
