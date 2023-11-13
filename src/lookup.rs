use color_eyre::eyre::{Result, WrapErr};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LookupEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description_short: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Lookup {
    entries: HashMap<String, LookupEntry>,
    #[serde(default)]
    pub loaded: bool,
    #[serde(default)]
    load_path: PathBuf,
}

impl Lookup {
    pub fn new(load_path: PathBuf) -> Self {
        Self {
            entries: HashMap::new(),
            loaded: false,
            load_path,
        }
    }
    pub fn load(&mut self) -> Result<()> {
        let files = std::fs::read_dir(self.load_path.as_path()).wrap_err_with(|| format!("failed to read lookups from '{}'", self.load_path.to_string_lossy()))?;

        files
            .filter_map(|f| match f {
                Ok(file) => {
                    if (match file.file_type() {
                        Ok(t) => t.is_file(),
                        Err(_) => return None,
                    }) && (file.file_name().to_str().unwrap().ends_with(".json"))
                    {
                        Some(file)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            .for_each(|file| {
                let lookup: Lookup = serde_json::from_slice(
                    std::fs::read(file.path())
                        .wrap_err_with(|| {
                            format!(
                                "failed to read lookup file '{}'",
                                file.path().to_string_lossy()
                            )
                        })
                        .unwrap()
                        .as_slice(),
                )
                .wrap_err_with(|| {
                    format!(
                        "failed to parse lookup file '{}'",
                        file.path().to_string_lossy()
                    )
                })
                .unwrap();
                self.entries.extend(lookup.entries.into_iter());
            });

        Ok(())
    }

    pub fn get_entry(&self, name: &str) -> Option<&LookupEntry> {
        self.entries.get(name)
    }
}
