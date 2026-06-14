use std::io::Result;

use crate::{
    cmis::{self},
    http::{HttpSession, SecretProvider},
    repl::def::{
        Repl,
        ReplState::{self, Continue},
        Session,
    },
};

impl Repl {
    pub(super) fn command_login(
        &mut self,
        list: bool,
        new_connection: Option<String>,
        repo: Option<&String>,
        interactive: bool,
    ) -> Result<ReplState> {
        if list {
            let mut names = self.cfg.connections.keys().collect::<Vec<_>>();
            names.sort();
            Ok(Continue(names.iter().map(|k| k.to_string()).collect()))
        } else {
            let connection_name = new_connection.unwrap();
            match self.cfg.connections.get(&connection_name) {
                Some(connection) => {
                    if interactive {
                        self.secret_cache.borrow_mut().invalidate_secret(connection);
                    }
                    self.session = Some(Session {
                        global: self.cfg.global.clone(),
                        connection: connection.clone(),
                        connection_name: connection_name.clone(),
                        http_session: HttpSession::new(
                            connection.clone(),
                            self.secret_cache.clone(),
                        ),
                        ..Default::default()
                    });
                    self.command_switch(false, repo)
                }
                None => io_error!(format!(
                    "connection configuration '{}' not found",
                    connection_name
                )),
            }
        }
    }

    pub(super) fn command_switch(
        &mut self,
        list: bool,
        repo: Option<&String>,
    ) -> Result<ReplState> {
        let session = get_session!(self);
        let repositories = cmis::get_repository_list(session)?;
        if list {
            let mut names = repositories
                .values()
                .map(|info| &info.repository_name)
                .collect::<Vec<_>>();
            names.sort();
            Ok(Continue(names.iter().map(|k| k.to_string()).collect()))
        } else {
            let new_repo_name = match repo {
                Some(name) => name,
                None => &session.connection.initial_repo,
            };
            match repositories
                .values()
                .find(|info| info.repository_name.eq_ignore_ascii_case(new_repo_name))
            {
                Some(info) => {
                    session.repository = info.to_owned();
                    self.command_cd("/")
                }
                None => io_error!(format!(
                    "repository '{new_repo_name}' not found in connection '{}'",
                    session.connection_name
                )),
            }
        }
    }
}
