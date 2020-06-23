use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters<'par> {
    path: &'par str,
}
pub fn build_project(parameters: Parameters) -> Result<(), anyhow::Error> {
    Ok(())
}
