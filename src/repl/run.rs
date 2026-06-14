use std::{io::Result, path::PathBuf, rc::Rc};

use clap::{Parser, error::ErrorKind::MissingSubcommand};
use rustyline::{DefaultEditor, error::ReadlineError};

use crate::{
    cmis::get_path,
    config::Config,
    repl::{
        cli::{Command, ReplCli},
        def::{
            Repl,
            ReplState::{self, Continue, Exit},
        },
    },
};

impl Repl {
    pub fn new(cfg: Rc<Config>) -> Self {
        Self {
            cfg,
            secret_cache: Default::default(),
            session: Default::default(),
        }
    }

    pub fn run(&mut self, history_file: &PathBuf) -> Result<()> {
        let mut editor = match DefaultEditor::new() {
            Ok(editor) => editor,
            Err(err) => return io_error!(err),
        };

        if let Err(err) = editor.load_history(history_file) {
            eprintln!("{err}");
        }

        loop {
            let prompt = match &self.session {
                Some(session) => {
                    let properties = session.cwd.get_dir_properties(session)?;
                    let path = get_path(&properties)?;
                    format!(
                        "{}:{}:{}> ",
                        session.connection_name, session.repository.repository_name, path
                    )
                }
                None => "> ".to_string(),
            };
            match editor.readline(prompt.as_str()) {
                Ok(line) => {
                    let _ = editor.add_history_entry(&line);
                    match self.evaluate(&line) {
                        Ok(Continue(output)) => {
                            output.iter().for_each(|s| println!("{s}"));
                        }
                        Ok(Exit) => break,
                        Err(err) => {
                            eprintln!("{err}");
                            // FIXME probably unnecessary/wrong
                            self.session = None
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => return io_error!(err),
            }
        }

        match editor.append_history(history_file) {
            Ok(()) => Ok(()),
            Err(err) => io_error!(err),
        }
    }

    fn evaluate(&mut self, line: &str) -> Result<ReplState> {
        match shlex::split(line) {
            Some(args) => match ReplCli::try_parse_from(args) {
                Ok(cli) => match cli.cmd {
                    Some(cmd) => self.dispatch_cmd(cmd, line),
                    None => Ok(Continue(vec![])),
                },
                Err(err) if err.kind() == MissingSubcommand => Ok(Continue(vec![])),
                Err(err) => io_error!(err),
            },
            None => io_error!("raw parsing error"),
        }
    }

    fn dispatch_cmd(&mut self, cmd: Command, line: &str) -> Result<ReplState> {
        match cmd {
            Command::Exit => Ok(Exit),
            Command::Clear => {
                DefaultEditor::new().unwrap().clear_screen().unwrap();
                Ok(Continue(vec![]))
            }
            Command::Login {
                list,
                connection,
                repo,
                interactive,
            } => self.command_login(list, connection, repo.as_ref(), interactive),
            Command::Switch { list, repo } => self.command_switch(list, repo.as_ref()),
            Command::Cd { path } => self.command_cd(&path),
            // TODO Command::Dir { path } => self.command_dir(&path),
            Command::Stat { path } => self.command_stat(&path),
            Command::Select { query: _ } => self.command_select(line),
            Command::Count { where_clause: _ } => not_implemented!(),
            Command::Local { cmd: _ } => not_implemented!(),
        }
    }
}
