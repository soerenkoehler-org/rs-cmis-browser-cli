use std::{cell::RefCell, env, fs::read_to_string, io::Result, rc::Rc, time::Instant};

use ureq::{
    Agent,
    tls::{Certificate, RootCerts, TlsConfig, TlsConfigBuilder},
};

use crate::http::def::{ENV_VAR_DEBUG_ROOT_CERT, HttpSession, Token};

// FIXME impl SecretProvider for SecretProviderDefault {
//     fn get_secret(&self, _connection: &ConfigConnection) -> String {
//         String::new()
//     }

//     fn invalidate_secret(&self, _connection: &ConfigConnection) {}
// }

impl Default for Token {
    fn default() -> Self {
        Self {
            current: Default::default(),
            created: Instant::now(),
            expires: 0, // default token expires immediately
        }
    }
}

impl Default for HttpSession {
    fn default() -> Self {
        Self {
            agent: Self::create_agent(),
            token: Default::default(),
        }
    }
}

impl HttpSession {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            ..Default::default()
        }
    }

    pub fn create_agent() -> Agent {
        let builder = TlsConfig::builder();
        let config = Self::load_debug_root_cert(builder).build();
        Agent::config_builder()
            .http_status_as_error(false)
            .tls_config(config)
            .build()
            .new_agent()
    }

    fn load_debug_root_cert(tls_cfg: TlsConfigBuilder) -> TlsConfigBuilder {
        match env::var(ENV_VAR_DEBUG_ROOT_CERT) {
            Ok(cert_file) => {
                let cert = Self::load_pem(&cert_file).unwrap();
                tls_cfg.root_certs(RootCerts::new_with_certs(&[cert]))
            }
            _ => tls_cfg,
        }
    }

    fn load_pem<'a>(path: &str) -> Result<Certificate<'a>> {
        let pem = read_to_string(path)?;
        match Certificate::from_pem(pem.as_bytes()) {
            Ok(cert) => Ok(cert),
            Err(err) => io_error!(err),
        }
    }
}
