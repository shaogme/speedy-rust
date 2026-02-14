use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to output the versions JSON file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Path to input the previous versions JSON file (for comparison)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Path to output the update status JSON
    #[arg(short = 's', long)]
    update_status: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    pkg: Pkg,
}

#[derive(Debug, Deserialize)]
struct Pkg {
    rust: RustPkg,
}

#[derive(Debug, Deserialize)]
struct RustPkg {
    version: String,
    git_commit_hash: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct OutputVersions {
    stable_version: String,
    stable_hash: String,
    nightly_version: String,
    nightly_hash: String,
    nightly_full_string: String,
}

#[derive(Debug, Serialize)]
struct CheckResult {
    stable_needs_update: bool,
    nightly_needs_update: bool,
    current: OutputVersions,
}

fn fetch_manifest(channel: &str) -> Result<RustPkg> {
    let url = format!(
        "https://static.rust-lang.org/dist/channel-rust-{}.toml",
        channel
    );
    let resp = reqwest::blocking::get(&url)?.text()?;
    let manifest: Manifest = toml::from_str(&resp)?;
    Ok(manifest.pkg.rust)
}

fn parse_version_number(version_str: &str) -> String {
    // Expected format: "1.76.0 (hash date)" or "1.78.0-nightly (hash date)"
    version_str
        .split_whitespace()
        .next()
        .unwrap_or(version_str)
        .to_string()
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Fetching stable manifest...");
    let stable = fetch_manifest("stable").context("Failed to fetch stable manifest")?;
    println!("Fetching nightly manifest...");
    let nightly = fetch_manifest("nightly").context("Failed to fetch nightly manifest")?;

    let stable_ver_num = parse_version_number(&stable.version);
    let nightly_ver_num = parse_version_number(&nightly.version);

    let current_state = OutputVersions {
        stable_version: stable_ver_num,
        stable_hash: stable.git_commit_hash,
        nightly_version: nightly_ver_num, // clean version e.g. 1.78.0-nightly
        nightly_hash: nightly.git_commit_hash,
        nightly_full_string: nightly.version, // full string with date
    };

    let mut stable_needs_update = true;
    let mut nightly_needs_update = true;

    // Try to read previous state
    if let Some(input_path) = &args.input {
        if input_path.exists() {
            println!("Reading previous state from {:?}", input_path);
            let content = fs::read_to_string(input_path)?;
            if let Ok(previous_state) = serde_json::from_str::<OutputVersions>(&content) {
                if previous_state.stable_hash == current_state.stable_hash {
                    stable_needs_update = false;
                    println!("Stable is up to date.");
                } else {
                    println!(
                        "Stable update found: {} -> {}",
                        previous_state.stable_version, current_state.stable_version
                    );
                }

                if previous_state.nightly_hash == current_state.nightly_hash {
                    nightly_needs_update = false;
                    println!("Nightly is up to date.");
                } else {
                    println!(
                        "Nightly update found: {} -> {}",
                        previous_state.nightly_version, current_state.nightly_version
                    );
                }
            } else {
                eprintln!("Failed to parse previous state file, assuming update needed.");
            }
        }
    }

    // Generate output JSON for state recording
    let state_json = serde_json::to_string_pretty(&current_state)?;
    println!("Current State:\n{}", state_json);

    if let Some(path) = args.output {
        fs::write(path, state_json).context("Failed to write output file")?;
    }

    // Generate output JSON for update status
    if let Some(status_path) = args.update_status {
        let status = CheckResult {
            stable_needs_update,
            nightly_needs_update,
            current: current_state, // include current info in status for convenience
        };
        let status_json = serde_json::to_string_pretty(&status)?;
        fs::write(&status_path, status_json).context("Failed to write status file")?;
        println!("Update status written to {:?}", status_path);
    }

    Ok(())
}
