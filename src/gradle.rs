use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

pub fn run(working_dir: &Path, command: &str, verbose: bool) -> Result<(), anyhow::Error> {
    let revert_dir = std::env::current_dir()?;
    std::env::set_current_dir(working_dir)?;
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
    let mut child = if verbose {
        std::process::Command::new(gradlew)
            .args(&[command])
            .spawn()?
    } else {
        spinner.set_message("Running...");
        std::process::Command::new(gradlew)
            .args(&[command])
            .stdout(std::process::Stdio::null())
            .spawn()?
    };
    while let Ok(None) = child.try_wait() {
        if !verbose {
            spinner.tick();
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    if !verbose {
        spinner.finish_and_clear();
    }
    std::env::set_current_dir(revert_dir)?;
    Ok(())
}
