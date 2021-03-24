mod manifests;

use crate::manifests::v0_3::{AssetLoaderID, StageConversion};
use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs::{self, File},
    path::Path,
};

fn main() {
    let raw = {
        let mut args = env::args();
        args.next();
        args.next().expect("No input file provided.")
    };
    let input = Path::new(&raw);

    let new = {
        let old: manifests::v0_2::Manifest = {
            let file = File::open(input).expect("Failed to open old manifest");

            serde_json::from_reader(file).expect("Failed to parse old manifest.")
        };

        fn insert_conv(
            map: &mut HashMap<String, AssetLoaderID>,
            old: &str,
            r#mod: &str,
            new: &str,
        ) {
            assert!(map
                .insert(
                    old.to_owned(),
                    AssetLoaderID(r#mod.to_owned(), new.to_owned())
                )
                .is_none());
        }

        let convs = StageConversion {
            patcher: {
                let mut map = HashMap::new();

                insert_conv(&mut map, "assembly", "deli", "assembly");
                insert_conv(&mut map, "monomod", "deli", "monomod");

                map
            },
            runtime: {
                let mut map = HashMap::new();

                insert_conv(&mut map, "assembly", "deli", "assembly");

                insert_conv(&mut map, "Character", "h3vr.tnhtweaker.deli", "character");
                insert_conv(&mut map, "Sosig", "h3vr.tnhtweaker.deli", "sosig");
                insert_conv(&mut map, "VaultFile", "h3vr.tnhtweaker.deli", "vault_file");

                map
            },
        };

        manifests::v0_3::Manifest::from(old, &convs)
    };

    {
        fs::create_dir_all("out").expect("Failed to create output directory.");

        let name = {
            let mut name = OsString::from("out/");
            name.push(format!("{}.json", new.guid));

            name
        };
        let file = File::create(&name).expect("Failed to open new manifest");

        serde_json::to_writer_pretty(file, &new).expect("Failed to serialize new manifest");
    }
}
