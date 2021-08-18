use clap::{AppSettings, Clap};
use dirs;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};
use url::Url;

const CACHE_DIR: &str = ".resorepo";
const CONFIG_FILE_NAME: &str = "resorepo_config.yaml";
// Can't include www, causes issues cloning
const GITHUB_BASE_URL: &str = "https://github.com";

#[derive(Clap)]
#[clap(version = "1.0")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Arguments {
    /// URL of remote repository
    repo_url: String,
    /// Branch or tag to check out
    #[clap(short, long)]
    branch: Option<String>,
    /// Args to pass to rg
    #[clap(min_values = 1)]
    rg_args: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    cache_ttl_days: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self { cache_ttl_days: 7 }
    }
}

fn main() {
    let args = Arguments::parse();
    let cache_path = initialize_cache();
    let config_file_path = cache_path.join(CONFIG_FILE_NAME);
    let config = get_config(&config_file_path);
    let mut parsed_url = parse_url(&args.repo_url);
    let repo_name = get_repo_name_from_url(&mut parsed_url);
    let repo_path = cache_path.join(repo_name);
    Repository::clone(parsed_url.as_str(), &repo_path).expect("Failed to clone");
    if let Some(branch) = &args.branch {
        dbg!(branch);
    };
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

fn read_config(config_file_path: &PathBuf) -> Config {
    let contents = fs::read_to_string(config_file_path).expect("Could not load config file");
    serde_yaml::from_str(&contents)
        .expect("Could not deserialize the config, make sure it is valid")
}

fn create_default_config(config_file_path: &PathBuf) {
    let config = Config::default();
    let serialized_config =
        serde_yaml::to_string(&config).expect("Could not serialize default config");
    fs::write(config_file_path, serialized_config).expect("Could not write default config");
}

fn get_config(config_file_path: &PathBuf) -> Config {
    if !config_file_path.exists() {
        create_default_config(config_file_path);
    } else {}
    read_config(config_file_path)
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
