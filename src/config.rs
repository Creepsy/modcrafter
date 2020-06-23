use super::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    major: u16,
    minor: u16,
    revision: u16,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ModConfig {
    pub mc_version: Version,
    pub mod_version: String,
    pub display_name: String,
    pub description: String,
    pub mod_id: String,
}

impl ModConfig {
    pub fn new(
        directory: &Path,
        display_name: Option<&str>,
        mod_id: Option<&str>,
        mod_version: Option<&str>,
        description: Option<&str>,
    ) -> Result<Self, Error> {
        std::fs::create_dir_all(directory)?;
        let p_abs = directory.canonicalize()?;
        let display_name = if let Some(name) = display_name {
            String::from(name)
        } else {
            String::from(
                p_abs
                    .file_name()
                    .ok_or(Error::FileIOError)?
                    .to_string_lossy(),
            )
        };
        let mod_id = if let Some(modid) = mod_id {
            String::from(modid)
        } else {
            display_name.replace(" ", "_").to_lowercase()
        };
        let mod_version = String::from(if let Some(version) = mod_version {
            version
        } else {
            "0.1.0"
        });
        let description = String::from(if let Some(description) = description {
            description
        } else {
            "This mod was created using modcrafter."
        });
        Ok(ModConfig {
            mc_version: Version::empty(),
            mod_id,
            mod_version,
            description,
            display_name: String::from(display_name),
        })
    }

    pub fn set_mc_version(&mut self, verstr: &str) {
        self.mc_version = Version::from(verstr);
    }
}
