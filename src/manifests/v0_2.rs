use crate::manifests::FieldMap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Manifest {
    pub guid: String,
    pub version: String,

    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub authors: Option<Vec<String>>,
    #[serde(rename = "sourceUrl")]
    #[serde(default)]
    pub source_url: Option<String>,

    #[serde(default)]
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(default)]
    pub patcher: Option<FieldMap<String>>,
    #[serde(default)]
    pub runtime: Option<FieldMap<String>>,
}
