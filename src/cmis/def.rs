use std::{collections::HashMap, io::Error};

use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const CMIS_OBJECT_ID: &str = "objectId";

pub const CMIS_SELECTOR: &str = "cmisselector";
pub const CMIS_SELECTOR_QUERY: &str = "query";
pub const CMIS_SELECTOR_CHILDREN: &str = "children";
pub const CMIS_SELECTOR_PROPERTIES: &str = "properties";

pub const CMIS_QUERY: &str = "q";

pub const CMIS_MAX_ITEMS: &str = "maxItems";
pub const CMIS_SKIP_COUNT: &str = "skipCount";
pub const CMIS_SUCCINCT: &str = "succinct";

pub const CMIS_FILTER: &str = "filter";

pub const CMIS_DIR_ATTRIBUTES: &str = concat!(
    "cmis:objectId,",
    "cmis:objectTypeId,",
    "cmis:name,",
    "cmis:parentId,",
    "cmis:path,",
    "path,",
    "cmis:contentStreamLength",
);

pub const CMIS_TYPE_ID_FOLDER: &str = "cmis:folder";
pub const CMIS_TYPE_ID_DOCUMENT: &str = "cmis:document";

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub repository_name: String,
    // pub repository_url: String,
    pub root_folder_id: String,
    pub root_folder_url: String,
}
pub type RepositoryList = HashMap<String, Repository>;

pub type PropertyList = HashMap<String, Value>;

#[derive(Clone, Default)]
pub struct ObjectId {
    pub id: String,
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub succinct_properties: PropertyList,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultsResponse {
    pub results: Vec<Object>,
    pub _has_more_items: bool,
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    pub object: Object,
    pub _path_segment: String,
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildrenResponse {
    pub objects: Vec<Child>,
    pub _has_more_items: bool,
}

pub struct PageReader<T> {
    pub progress: ProgressBar,
    pub error: Option<Error>,
    pub objects: Vec<Vec<T>>,
}
