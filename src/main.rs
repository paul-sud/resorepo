use clap::{AppSettings, Clap};
use dirs;
use git2::Repository;
use std::{fs, path::PathBuf, process::Command};
use url::Url;

const CACHE_DIR: &str = ".resorepo";
const GITHUB_BASE_URL: &str = "https://github.com";

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
    let mut parsed_url = parse_url(&args.repo_url);
    let repo_name = get_repo_name_from_url(&mut parsed_url);
    let repo_path = cache_path.join(repo_name);
    Repository::clone(parsed_url.as_str(), &repo_path).expect("Failed to clone");
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

fn parse_url(repo_url: &str) -> Url {
    match Url::parse(repo_url) {
        Ok(result) => result,
        Err(_) => Url::parse(GITHUB_BASE_URL).unwrap().join(repo_url).unwrap(),
    }
}

fn get_repo_name_from_url(parsed_url: &mut Url) -> String {
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
    fn test_parse_url() {
        let url = "https://www.github.com/foo/bar";
        let result = parse_url(url);
        assert_eq!(result.as_str(), "https://www.github.com/foo/bar")
    }

    #[test]
    fn test_parse_url_no_base() {
        let url = "foo/bar";
        let result = parse_url(url);
        assert_eq!(result.as_str(), "https://www.github.com/foo/bar")
    }

    #[test]
    fn test_get_repo_name_from_url() {
        let mut url = Url::parse("https://www.github.com/foo/bar").unwrap();
        let result = get_repo_name_from_url(&mut url);
        assert_eq!(result, "bar".to_owned())
    }

    #[test]
    fn test_get_repo_name_from_url_trailing_slash() {
        let mut url = Url::parse("https://www.github.com/foo/bar/").unwrap();
        let result = get_repo_name_from_url(&mut url);
        assert_eq!(result, "bar".to_owned())
    }
}
