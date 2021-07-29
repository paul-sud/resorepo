use clap::{AppSettings, Clap};
use dirs;
use git2::Repository;
use std::{fs, path::PathBuf, process::Command};
use url::Url;

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
    let cache_path = initialize_cache();
    let args = Arguments::parse();
    let repo_name = get_repo_name_from_url(&args.repo_url);
    let repo_path = cache_path.join(repo_name);
    Repository::clone(&args.repo_url, &repo_path).expect("Failed to clone");
    let mut command = Command::new("rg");
    for rg_arg in args.rg_args.iter() {
        command.arg(rg_arg);
    }
    command.arg(&repo_path);
    let status = command.status().expect("Failed to execute process");
    if !status.success() {
        println!("rg failed");
    }
}

fn initialize_cache() -> PathBuf {
    let mut cache_dir = dirs::home_dir().expect("Could not determine home directory");
    cache_dir.push(CACHE_DIR);
    if !cache_dir.exists() {
        fs::create_dir(&cache_dir).expect("Could not create cache");
    }
    cache_dir
}

fn get_repo_name_from_url(repo_url: &str) -> String {
    let mut parsed_url = Url::parse(repo_url).expect("Could not parse url");
    parsed_url.path_segments_mut().unwrap().pop_if_empty();
    parsed_url
        .path_segments()
        .unwrap()
        .last()
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_repo_name_from_url() {
        let url = "https://www.github.com/foo/bar";
        let result = get_repo_name_from_url(url);
        assert_eq!(result, "bar".to_owned())
    }

    #[test]
    fn test_get_repo_name_from_url_trailing_slash() {
        let url = "https://www.github.com/foo/bar/";
        let result = get_repo_name_from_url(url);
        assert_eq!(result, "bar".to_owned())
    }
}
