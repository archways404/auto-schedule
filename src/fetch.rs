use std::process::Command;
use std::error::Error;
use std::env;

const SCRIPT_PATH: &str = "./fetch_ics.sh";

pub async fn fetch_ics() -> Result<(), Box<dyn Error>> {
    // Print current working directory for debugging
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    let output = Command::new(SCRIPT_PATH)
        .output()
        .expect("Failed to execute script");

    if !output.status.success() {
        return Err(format!("Script failed with output: {:?}", output).into());
    }

    println!("ICS file fetched successfully");
    Ok(())
}
