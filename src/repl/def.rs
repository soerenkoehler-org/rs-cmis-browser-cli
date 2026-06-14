use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rustyline::{Completer, Helper, Hinter, Validator};

use crate::{
    cmis::{ObjectId, Repository},
    config::{Config, ConfigConnection, ConfigGlobal},
    http::HttpSession,
};

/// Controls the continuation of the REPL
pub enum ReplState {
    Continue(Vec<String>),
    Exit,
}

/// Global state for the REPL
pub struct Repl {
    pub(super) cfg: Rc<Config>,
    pub(super) secret_cache: RefCell<SecretCache>,
    pub(super) session: Option<Session>,
}

/// Secretprovider which ask for missing secrets and caches them
#[derive(Default)]
pub(super) struct SecretCache {
    pub secrets: HashMap<String, SecretState>,
}

/// State of a known secret
pub(super) enum SecretState {
    /// The secret is already cached
    Cached(String),
    /// Ask the user for a new secret value
    Interactive,
}

/// Rustyline helper object for password input in SecretCache
#[derive(Completer, Helper, Hinter, Validator)]
pub(super) struct MaskingHighlighter {}

/// Session state
#[derive(Default)]
pub struct Session { // FIXME => CMIS-Session
    pub config: Rc<Config>,
    pub connection_name: String,
    // mutable
    pub repository: Repository,
    pub cwd: ObjectId,
    pub http_session: HttpSession,
}
