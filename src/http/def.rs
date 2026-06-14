extern crate proc_macro;

use std::time::Instant;

use serde::Deserialize;
use ureq::Agent;

pub const ENV_VAR_DEBUG_ROOT_CERT: &str = "DEBUG_ROOT_CERT";

pub const HEADER_AUTH: &str = "Authorization";

pub struct HttpSession {
    pub agent: Agent,
    pub token: Token,
}

// pub struct SecretProviderDefault {}

pub struct Token {
    pub(super) current: String,
    pub(super) created: Instant,
    pub(super) expires: u64,
}

#[derive(Deserialize)]
pub(super) struct TokenResponse {
    pub(super) access_token: String,
    pub(super) expires_in: u64,
}
