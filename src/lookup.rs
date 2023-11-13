use color_eyre::eyre::{Result, WrapErr};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SpellEntry {
    #[serde(default)]
    pub spell: String,
    #[serde(default)]
    pub description_short: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Lookup {
    spells: HashMap<String, SpellEntry>,
}

impl Lookup {
    pub fn load(directory: &Path) -> Result<Self> {
        let mut main_lookup = Lookup {
            spells: HashMap::new(),
        };
        let files = std::fs::read_dir(directory).wrap_err_with(|| format!("failed to read lookups from '{}'", directory.to_string_lossy()))?;

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
                main_lookup.spells.extend(lookup.spells.into_iter());
            });

        Ok(main_lookup)
    }

    pub fn get_spell(&self, spell: &str) -> Option<&SpellEntry> {
        self.spells.get(spell)
    }
}
