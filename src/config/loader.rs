use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufReader, BufWriter, Error, ErrorKind, Result},
    path::PathBuf,
    rc::Rc,
};

use crate::config::{
    ConfigGlobal,
    def::{Config, ConfigAuth, ConfigConnection, ConfigConnectionMap},
};

impl Default for Config {
    fn default() -> Self {
        Config {
            global: Rc::new(ConfigGlobal {
                paging_size: 100000,
                ..Default::default()
            }),
            connections: HashMap::from([(
                "example".to_string(),
                Rc::new(ConfigConnection {
                    name: "example".to_string(),
                    url: "https://example.io/browser".to_string(),
                    initial_repo: "R1".to_string(),
                    auth: ConfigAuth {
                        issuer: "issuerUrl".to_string(),
                        client_id: "clientID".to_string(),
                        client_secret: "clientSecret".to_string(),
                        expiration_grace_period: 30,
                    },
                    extra_headers: HashMap::from([(
                        "Ocp-Apim-Subscription-Key".to_string(),
                        "subscriptionKey".to_string(),
                    )]),
                }),
            )]),
        }
    }
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Rc<Config>> {
        let mut cfg: Config = match OpenOptions::new().read(true).open(path) {
            Ok(file) => match serde_json::from_reader(BufReader::new(file)) {
                Ok(cfg) => Ok(cfg),
                Err(err) => Err(Error::other(err.to_string())),
            },
            Err(err) if err.kind() == ErrorKind::NotFound => Config::default().save(path),
            Err(err) => Err(err),
        }?;

        cfg.connections =
            cfg.connections
                .iter()
                .fold(ConfigConnectionMap::new(), |mut m, (k, v)| {
                    let mut connection = ConfigConnection::clone(v);
                    connection.name = k.to_string();
                    m.insert(k.to_string(), Rc::new(connection));
                    m
                });

        Ok(Rc::new(cfg))
    }

    fn save(self, path: &PathBuf) -> Result<Config> {
        match OpenOptions::new().create_new(true).write(true).open(path) {
            Ok(file) => match serde_json::to_writer_pretty(BufWriter::new(file), &self) {
                Ok(_) => Ok(self),
                Err(err) => Err(Error::other(err.to_string())),
            },
            Err(err) => Err(err),
        }
    }
}
