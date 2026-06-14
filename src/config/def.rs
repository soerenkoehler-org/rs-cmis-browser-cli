use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Config {
    pub global: Rc<ConfigGlobal>,
    pub connections: ConfigConnectionMap,
}

pub type ConfigConnectionMap = HashMap<String, Rc<ConfigConnection>>;

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ConfigGlobal {
    pub paging_size: usize,
    pub get_children_method: GetChildrenMethod,
}

#[derive(Default, Deserialize, Serialize)]
pub enum GetChildrenMethod {
    #[default]
    Selector,
    Query
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ConfigConnection {
    #[serde(skip)]
    pub name: String,
    pub url: String,
    pub initial_repo: String,
    pub auth: ConfigAuth,
    pub extra_headers: HashMap<String, String>,
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ConfigAuth {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub expiration_grace_period: u64,
}
