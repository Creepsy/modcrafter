use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    major: u16,
    minor: u16,
    revision: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModConfig {
    mc_version: Version,
    mod_version: String,
    display_name: String,
    description: String,
    mod_id: String,
    authors: Option<String>,
}

impl Version {
    fn empty() -> Self {
        Version {
            major: 0,
            minor: 0,
            revision: 0,
        }
    }
}

impl From<&str> for Version {
    fn from(verstr: &str) -> Self {
        let mut split = verstr.split(".");
        Version {
            major: split.next().unwrap().parse().unwrap(),
            minor: split.next().unwrap().parse().unwrap(),
            revision: split.next().unwrap().parse().unwrap(),
        }
    }
}

impl ModConfig {
    pub fn new(
        directory: &Path,
        display_name: Option<&str>,
        mod_id: Option<&str>,
        mod_version: Option<&str>,
        description: Option<&str>,
        authors: Option<&str>,
    ) -> Result<Self, anyhow::Error> {
        std::fs::create_dir_all(directory)?;
        let p_abs = directory.canonicalize()?;
        let display_name = if let Some(name) = display_name {
            String::from(name)
        } else {
            String::from(
                p_abs
                    .file_name()
                    .ok_or(anyhow::Error::new(std::io::Error::from_raw_os_error(2)))?
                    .to_string_lossy(),
            )
        };
        let mod_id = if let Some(modid) = mod_id {
            String::from(modid)
        } else {
            display_name.replace(" ", "_").to_lowercase()
        };
        let mod_version = String::from(mod_version.unwrap_or("0.1.0"));
        let description =
            String::from(description.unwrap_or("This mod was created using modcrafter."));
        Ok(ModConfig {
            mc_version: Version::empty(),
            mod_id,
            mod_version,
            description,
            display_name: String::from(display_name),
            authors: if let Some(authors) = authors {
                Some(String::from(authors))
            } else {
                None
            },
        })
    }

    pub fn set_mc_version(&mut self, verstr: &str) {
        self.mc_version = Version::from(verstr);
    }

    pub fn mod_id(&self) -> &str {
        &self.mod_id
    }

    pub fn mod_version(&self) -> &str {
        &self.mod_version
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}
