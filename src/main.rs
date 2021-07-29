use clap::{AppSettings, Clap};
use dirs;
use git2::Repository;
use std::{fs, path::PathBuf, process::Command};

const CACHE_DIR: &str = ".resorepo";

#[derive(Clap)]
#[clap(version = "1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Arguments {
    /// URL of remote repository
    repo_url: String,
    /// Args to pass to rg
    #[clap(min_values = 1)]
    rg_args: Vec<String>,
}

fn main() {
    // initialize_cache();
    let args = Arguments::parse();
    let repo_path = "repopath";
    let repo = match Repository::clone(&args.repo_url, repo_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };
    let mut command = Command::new("rg");
    for rg_arg in args.rg_args.iter() {
        command.arg(rg_arg);
    }
    command.arg(repo_path);
    let status = command.status().expect("failed to execute process");
    if !status.success() {
        println!("rg failed");
    }
}

fn initialize_cache() -> PathBuf {
    let mut cache_dir = dirs::home_dir().expect("Could not determine home directory");
    cache_dir.push(CACHE_DIR);
    if !cache_dir.exists() {
        fs::create_dir("/some/dir").expect("Could not create cache");
    }
    cache_dir
}
