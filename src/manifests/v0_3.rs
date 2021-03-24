use super::v0_2::Manifest as OldManifest;
use crate::manifests::{Field, FieldMap};
use serde::{Serialize, Serializer};
use std::collections::HashMap;

#[derive(Clone)]
pub struct AssetLoaderID(pub String, pub String);

impl Serialize for AssetLoaderID {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&format_args!("{}:{}", self.0, self.1))
    }
}

type AssetMap = FieldMap<AssetLoaderID>;

#[derive(Serialize, Default)]
pub struct AssetTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patcher: Option<AssetMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup: Option<AssetMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AssetMap>,
}

#[derive(Serialize)]
pub struct Manifest {
    pub guid: String,
    pub version: String,
    pub require: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<AssetTable>,
}

pub struct StageConversion {
    pub patcher: HashMap<String, AssetLoaderID>,
    pub runtime: HashMap<String, AssetLoaderID>,
}

impl Manifest {
    pub fn from(old: OldManifest, conversions: &StageConversion) -> Self {
        let mut table: Option<AssetTable> = None;

        fn conv(
            entries: FieldMap<String>,
            conversions: &HashMap<String, AssetLoaderID>,
        ) -> Option<AssetMap> {
            let mut map: Option<AssetMap> = None;

            for Field(path, loader) in entries.0.into_iter() {
                let converted = if let Some(v) = conversions.get(&loader) {
                    v
                } else {
                    panic!(format!("Conversion not found for 0.2 loader: {}", loader))
                };

                map.get_or_insert_with(Default::default)
                    .0
                    .push(Field(path, converted.clone()));
            }

            map
        }

        fn conv_map<S: FnOnce(&mut AssetTable, Option<AssetMap>)>(
            entries: Option<FieldMap<String>>,
            table: &mut Option<AssetTable>,
            setter: S,
            conversions: &HashMap<String, AssetLoaderID>,
        ) {
            if let Some(v) = entries {
                setter(
                    table.get_or_insert_with(Default::default),
                    conv(v, &conversions),
                );
            }
        }

        conv_map(
            old.patcher,
            &mut table,
            |t, m| t.patcher = m,
            &conversions.patcher,
        );
        conv_map(
            old.runtime,
            &mut table,
            |t, m| t.setup = m,
            &conversions.runtime,
        );

        Manifest {
            guid: old.guid,
            version: old.version,
            require: "0.3.1".to_owned(),

            name: old.name,
            description: None,
            authors: old.authors,
            source_url: old.source_url,

            dependencies: old
                .dependencies
                .map(|mut v| {
                    v.remove("deli.core");
                    v.remove("deli.monomod");

                    if v.is_empty() {
                        None
                    } else {
                        Some(v)
                    }
                })
                .flatten(),
            assets: table,
        }
    }
}
