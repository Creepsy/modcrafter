use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    major: u16,
    minor: u16,
    revision: u16,
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
pub struct MainConfig {
    version: Version,
    name: String,
}

impl MainConfig {
    pub fn new(name: &str, version: &str) -> Self {
        MainConfig {
            version: Version::from(version),
            name: String::from(name),
        }
    }
}
