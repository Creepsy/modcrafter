use super::config::ModConfig;
use indicatif::ProgressBar;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters<'par> {
    directory: &'par str,
    forge_zip: &'par str,
    display_name: Option<&'par str>,
    mod_id: Option<&'par str>,
    mod_version: Option<&'par str>,
    description: Option<&'par str>,
    authors: Option<&'par str>,
}

impl<'par> Parameters<'par> {
    pub fn new(
        directory: &'par str,
        forge_zip: &'par str,
        display_name: Option<&'par str>,
        mod_id: Option<&'par str>,
        mod_version: Option<&'par str>,
        description: Option<&'par str>,
        authors: Option<&'par str>,
    ) -> Self {
        Parameters {
            directory,
            forge_zip,
            display_name,
            mod_id,
            mod_version,
            description,
            authors,
        }
    }
}

pub fn create_project(parameters: Parameters, verbose: bool) -> Result<(), anyhow::Error> {
    let p = Path::new(parameters.directory);
    if p.exists() {
        return Err(anyhow::Error::new(std::io::Error::from_raw_os_error(17)));
    }
    println!("[1/5] Creating Project Folder...");
    let mut config = ModConfig::new(
        p,
        parameters.display_name,
        parameters.mod_id,
        parameters.mod_version,
        parameters.description,
        parameters.authors,
    )?;
    let mod_dir = p.join(config.mod_id());
    fs::create_dir_all(&mod_dir)?;
    println!("[2/5] Unpacking Forge...");
    unpack_forge(parameters.forge_zip, &mod_dir)?;
    println!("[3/5] Writing Config...");
    let config_file = File::create(&p.join("config.json"))?;
    config.set_mc_version(&find_version_string(&mod_dir.join("build.gradle"))?);
    serde_json::to_writer_pretty(config_file, &config)?;
    println!("[4/5] Cleaning Example Files...");
    clean_example_files(&mod_dir, config.mod_id(), config.mod_version())?;
    println!("[5/5] Running Gradle...");
    crate::gradle::run(&mod_dir, "prepareRuns", verbose)?;
    Ok(())
}

fn find_version_string(p: &Path) -> Result<String, anyhow::Error> {
    let mut s = String::new();
    File::open(p)?.read_to_string(&mut s)?;
    let pattern = Regex::new(r"net\.minecraftforge:forge:.*-")?;
    let verstr_raw: &str = &pattern.captures_iter(&s).next().unwrap()[0];
    Ok(String::from(&verstr_raw[25..verstr_raw.len() - 1]))
}

fn unpack_forge(forge_zip: &str, mod_dir: &Path) -> Result<(), anyhow::Error> {
    let mut archive = zip::ZipArchive::new(File::open(forge_zip)?)?;
    let bar = ProgressBar::new(archive.len() as u64);
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file.sanitized_name();

        if file.name().ends_with('/') {
            fs::create_dir_all(&mod_dir.join(&outpath))?;
        } else {
            let mut outfile = fs::File::create(&mod_dir.join(&outpath))?;
            io::copy(&mut file, &mut outfile)?;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&mod_dir.join(&outpath), fs::Permissions::from_mode(mode))?;
            }
        }
        bar.inc(1);
    }
    bar.finish_and_clear();
    Ok(())
}

fn clean_example_files(
    mod_dir: &Path,
    mod_id: &str,
    mod_version: &str,
) -> Result<(), anyhow::Error> {
    fs::remove_dir_all(mod_dir.join("src"))?;
    let mut buf = String::new();
    {
        let mut build_gradle = File::open(mod_dir.join("build.gradle"))?;
        build_gradle.read_to_string(&mut buf)?;
        buf = buf.replace("com.yourname.modid", &format!("modcrafter.{}", mod_id));
        buf = buf.replace("modid", mod_id);
        buf = buf.replace("examplemod", mod_id);
        buf = buf.replace("version = '1.0'", &format!("version = '{}'", mod_version));
    }
    {
        let mut build_gradle = File::create(mod_dir.join("build.gradle"))?;
        build_gradle.write_all(buf.as_bytes())?;
    }
    Ok(())
}
