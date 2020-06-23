use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters<'par> {
    directory: &'par str,
}

impl<'par> Parameters<'par> {
    pub fn new(directory: &'par str) -> Self {
        Parameters { directory }
    }
}

fn create_pack_mcmeta(directory: &Path, display_name: &str) -> Result<(), anyhow::Error> {
    let mut file = File::create(directory.join("src/main/resources/pack.mcmeta"))?;
    let text = format!("{{\n    \"pack\": {{\n        \"description\": \"{} resources\",\n        \"pack_format\": 5,\n        \"_comment\": \"A pack_format of 5 requires json lang files and some texture changes from 1.15. Note: we require v5 pack meta for all mods.\"\n    }}\n}}\n", display_name);
    file.write_all(text.as_bytes())?;
    Ok(())
}

pub fn build_project(parameters: Parameters) -> Result<(), anyhow::Error> {
    let project_dir = Path::new(parameters.directory);
    let config_file = File::open(project_dir.join("config.json"))?;
    let config: crate::config::ModConfig = serde_json::from_reader(config_file)?;
    create_pack_mcmeta(&project_dir.join(config.mod_id()), config.display_name())?;
    Ok(())
}
