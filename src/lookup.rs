use color_eyre::eyre::{Result, WrapErr};
use serde_derive::Deserialize;
use std::{collections::HashMap, path::PathBuf, rc::Rc};

#[derive(Deserialize, Default, Clone)]
pub struct LookupEntry {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description_short: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Deserialize, Default)]
pub struct Lookup {
    entries: HashMap<String, Rc<LookupEntry>>,
    #[serde(default)]
    pub loaded: bool,
    #[serde(default)]
    load_path: PathBuf,
}

impl Lookup {
    /// Initializes a new, empty lookup table.
    ///
    /// `load_path` is the path to the directory containing all stored
    /// lookup tables that need to be loaded later.
    ///
    /// # Example
    /// ```no_run
    /// use scrim::lookup::Lookup;
    ///
    /// let mut lookup = Lookup::new("data/lookups".into());
    /// lookup.load().expect("failed to load lookup");
    /// lookup.get_entry("apple");
    /// ```
    pub fn new(load_path: PathBuf) -> Self {
        Self {
            entries: HashMap::new(),
            loaded: false,
            load_path,
        }
    }

    /// Load all lookup tables located in the directory specified by the load path.
    pub fn load(&mut self) -> Result<()> {
        let files = std::fs::read_dir(self.load_path.as_path()).wrap_err_with(|| {
            format!(
                "failed to read lookups from '{}'",
                self.load_path.to_string_lossy()
            )
        })?;

        files
            // filter for only JSON files
            .filter_map(|f| match f {
                Ok(file) => {
                    let path = file.path();
                    if path.is_file() && path.extension().unwrap_or_default() == "json" {
                        Some(path)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            })
            // try to parse each file
            .for_each(|path| {
                let lookup: Lookup = serde_json::from_slice(
                    std::fs::read(&path)
                        .wrap_err_with(|| {
                            format!(
                                "failed to read lookup file '{}'",
                                path.to_str().unwrap_or(&path.to_string_lossy())
                            )
                        })
                        .unwrap()
                        .as_slice(),
                )
                .wrap_err_with(|| {
                    format!(
                        "failed to parse lookup file '{}'",
                        path.to_str().unwrap_or(&path.to_string_lossy())
                    )
                })
                .unwrap();
                self.entries.extend(lookup.entries.into_iter());
            });

        Ok(())
    }

    /// Get the lookup entry with the current name, if it exists.
    pub fn get_entry(&self, name: &str) -> Option<&Rc<LookupEntry>> {
        self.entries.get(&name.to_lowercase())
    }

    /// Search the lookup table for all possible completions for the given text
    pub fn get_completions(&self, text: &str) -> Vec<Rc<LookupEntry>> {
        let text = text.trim_start().to_lowercase();
        self.entries
            .iter()
            .filter_map(|(k, v)| {
                if k.starts_with(&text) {
                    return Some(v.clone());
                }
                None
            })
            .collect()
    }
}
