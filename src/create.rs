use super::config::ModConfig;
use super::Error;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub fn create_project(
    directory: &str,
    forge_zip: &str,
    display_name: Option<&str>,
    modid: Option<&str>,
    modversion: Option<&str>,
    description: Option<&str>,
) -> Result<(), Error> {
    let p = Path::new(directory);
    if p.exists() {
        return Err(Error::ProjectFolderExists);
    }
    println!("[1/5] Creating Project Folder...");
    let mut config = ModConfig::new(p, display_name, modid, modversion, description)?;
    let mod_dir = p.join(&config.mod_id);
    fs::create_dir_all(&mod_dir)?;
    println!("[2/5] Unpacking Forge...");
    unpack_forge(forge_zip, &mod_dir)?;
    println!("[3/5] Writing Config...");
    let config_file = File::create(&p.join("config.json")).unwrap();
    config.set_mc_version(&find_version_string(&mod_dir.join("build.gradle"))?);
    serde_json::to_writer_pretty(config_file, &config).unwrap();
    println!("[4/5] Cleaning Example Files...");
    clean_example_files(&mod_dir, &config.mod_id, &config.mod_version)?;
    println!("[5/5] Running Gradle...");
    run_gradle(&mod_dir);
    Ok(())
}

fn find_version_string(p: &Path) -> Result<String, Error> {
    let mut s = String::new();
    File::open(p)?.read_to_string(&mut s)?;
    let pattern = Regex::new(r"net\.minecraftforge:forge:.*-").unwrap();
    let verstr_raw: &str = &pattern.captures_iter(&s).next().unwrap()[0];
    Ok(String::from(&verstr_raw[25..verstr_raw.len() - 1]))
}

fn run_gradle(forge_dir: &Path) {
    std::env::set_current_dir(forge_dir).unwrap();
    let gradlew;
    #[cfg(target_family = "unix")]
    {
        gradlew = "./gradlew";
    }
    #[cfg(target_family = "windows")]
    {
        gradlew = ".\\gradlew.bat";
    }
    let spinner = ProgressBar::new_spinner().with_style(
        ProgressStyle::default_spinner().template("[{elapsed_precise}] {msg} {spinner}"),
    );
    spinner.set_message("Running...");
    let mut child = std::process::Command::new(gradlew)
        .args(&["prepareRuns"])
        .stdout(std::process::Stdio::null())
        .spawn()
        .unwrap();
    while let Ok(None) = child.try_wait() {
        spinner.tick();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    spinner.finish_and_clear();
    std::env::set_current_dir("../").unwrap();
}

fn unpack_forge(forge_zip: &str, mod_dir: &Path) -> Result<(), Error> {
    let mut archive = zip::ZipArchive::new(File::open(forge_zip)?)?;
    let bar = ProgressBar::new(archive.len() as u64);
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

        if file.name().ends_with('/') {
            fs::create_dir_all(&mod_dir.join(&outpath)).unwrap();
        } else {
            let mut outfile = fs::File::create(&mod_dir.join(&outpath)).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&mod_dir.join(&outpath), fs::Permissions::from_mode(mode))
                    .unwrap();
            }
        }
        bar.inc(1);
    }
    bar.finish_and_clear();
    Ok(())
}

fn clean_example_files(mod_dir: &Path, mod_id: &str, mod_version: &str) -> Result<(), Error> {
    fs::remove_dir_all(mod_dir.join("src"))?;
    let mut buf = String::new();
    {
        let mut build_gradle = File::open(mod_dir.join("build.gradle")).unwrap();
        build_gradle.read_to_string(&mut buf).unwrap();
        buf = buf.replace("com.yourname.modid", &format!("modcrafter.{}", mod_id));
        buf = buf.replace("modid", mod_id);
        buf = buf.replace("examplemod", mod_id);
        buf = buf.replace("version = '1.0'", &format!("version = '{}'", mod_version));
    }
    {
        let mut build_gradle = File::create(mod_dir.join("build.gradle")).unwrap();
        build_gradle.write_all(buf.as_bytes()).unwrap();
    }
    Ok(())
}
