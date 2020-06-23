use super::config::MainConfig;
use super::Error;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

fn write_base_config(p: &Path, name: &str, version: &str) -> Result<(), Error> {
    let file = File::create(p)?;
    let config = MainConfig::new(name, version);
    serde_json::to_writer_pretty(file, &config).unwrap();
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
}

pub fn create_project(directory: &str, forge_zip: &str) -> Result<(), Error> {
    let p = Path::new(directory);
    if !p.exists() {
        println!("[1/4] Creating Project Folder...");
        fs::create_dir_all(p)?;
        fs::create_dir_all(p.join("mdkdir"))?;
        println!("[2/4] Unpacking Forge...");
        let mut archive = zip::ZipArchive::new(File::open(forge_zip)?)?;
        let bar = ProgressBar::new(archive.len() as u64);
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let outpath = file.sanitized_name();

            if file.name().ends_with('/') {
                fs::create_dir_all(p.join("mdkdir").join(&outpath)).unwrap();
            } else {
                let mut outfile = fs::File::create(p.join("mdkdir").join(&outpath)).unwrap();
                io::copy(&mut file, &mut outfile).unwrap();
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(
                        p.join("mdkdir").join(&outpath),
                        fs::Permissions::from_mode(mode),
                    )
                    .unwrap();
                }
            }
            bar.inc(1);
        }
        bar.finish_and_clear();
        println!("[3/4] Writing Config...");
        write_base_config(
            &p.join("config.json"),
            &*p.canonicalize()?
                .file_name()
                .ok_or(Error::FileIOError)?
                .to_string_lossy(),
            &find_version_string(&p.join("mdkdir").join("build.gradle"))?,
        )?;
        println!("[4/4] Running Gradle...");
        run_gradle(&p.join("mdkdir"));
    } else {
        return Err(Error::ProjectFolderExists);
    }
    Ok(())
}
