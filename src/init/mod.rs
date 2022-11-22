//TODO: Implement the badge
use inquire::{error::InquireError, Confirm, CustomType, Select, Text};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::canonicalize;
use std::fs::remove_file;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub mod init_content;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptConfig {
    pub config: crate::Config,
    pub contributor_file: String,
    pub badge_file: String,
}

pub fn init() {
    let config = prompt();

    let json = serde_json::to_string_pretty(&config.clone().config)
        .expect("Failed to serialize .all-contributorsrc");
    if PathBuf::from(".all-contributorsrc").exists() {
        remove_file(".all-contributorsrc").expect("Failed to generate .all-contributeorsrc");
    }
    let mut file = File::create(".all-contributorsrc").expect("Failed to open .all-contributorsrc");
    file.write(json.as_bytes())
        .expect("Failed to write JSON to .all-contributorsrc");

    ensure_file_exists(PathBuf::from(config.clone().contributor_file));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .read(true)
        .open(config.clone().contributor_file)
        .expect("Error opening contributor file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read contributor file");

    if !contents
        .contains("<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->")
    {
        let append = init_content::append_content();
        write!(file, "{}", append);
    }
}

fn ensure_file_exists(file: PathBuf) {
    if !file.exists() {
        if let Some(parent) = file.clone().parent() {
            fs::create_dir_all(&parent)
                .expect(&format!("Failed to create directory {:?}", parent).as_str());
        }
        let file = File::create(file.clone())
            .expect(&format!("Failed to create file {:?}", file).as_str());
    }
}

fn prompt() -> PromptConfig {
    // Some prompts remain unimplemented
    let list_of_commit_conventions: Vec<&str> = vec![
        "angular", "atom", "gitmoji", "ember", "eslint", "jshint", "none",
    ];
    let list_of_repo_types: Vec<&str> = vec!["github", "gitlab"];

    let project_name = match Text::new("What's the name of the repository?").prompt() {
        Ok(a) => a,
        Err(_) => panic!("Error processing project name"),
    };

    let project_owner = match Text::new("Who is the owner of the repository?").prompt() {
        Ok(a) => a,
        Err(_) => panic!("Error processing project name"),
    };

    let repo_type = match Select::new("What is the repository type?", list_of_repo_types).prompt() {
        Ok(a) => a.to_string(),
        Err(_) => panic!("Error processing repo type"),
    };

    let repo_host =
        match Text::new("Where is the repository hosted? Hit Enter if it's on GitHub or GitLab")
            .prompt()
        {
            Ok(a) => {
                if !(a == "".to_string()) {
                    a
                } else if repo_type == "gitlab" {
                    "https://gitlab.com".to_string()
                } else {
                    "https://github.com".to_string()
                }
            }
            Err(_) => panic!("Error processing repo_host"),
        };

    let contributor_file = match Text::new("In which file should contributors be listed?")
        .with_default("README.md")
        .prompt()
    {
        Ok(a) => a,
        Err(_) => panic!("Error processing contributing file"),
    };

    let badge = match Confirm::new("Do you want a badge tallying the number of contributors?")
        .with_default(false)
        .prompt()
    {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => panic!("Error processing badge"),
    };

    if badge {
        let badge_file = match Text::new("In which file should the badge be shown?").prompt() {
            Ok(a) => a,
            Err(_) => panic!("Error processing badge_file"),
        };
    }

    let image_size = match CustomType::<i64>::new("How big should the avatars be? (in px)")
        .with_formatter(&|i| format!("{}", i))
        .with_error_message("Please type a valid number")
        .with_default(100)
        .prompt()
    {
        Ok(a) => a,
        Err(_) => panic!("Error processing image size"),
    };

    let commit =
        match Confirm::new("Do you want this badge to auto-commit when contributors are added?")
            .prompt()
        {
            Ok(a) => a,
            Err(_) => panic!("Error processing auto-commit"),
        };

    let commit_convention = match Select::new(
        "What commit convention would you want it to use?",
        list_of_commit_conventions,
    )
    .prompt()
    {
        Ok(a) => a.to_string(),
        Err(_) => panic!("Error processing commit convention"),
    };

    let link_to_usage =
        match Confirm::new("Do you want to add a footer with link to usage").prompt() {
            Ok(a) => a,
            Err(_) => panic!("Error processing link to usage"),
        };

    PromptConfig {
        config: crate::Config {
            files: vec![contributor_file.clone()],
            image_size: image_size,
            commit: commit,
            commit_convention: commit_convention,
            contributors: Vec::new(),
            contributors_per_line: 7,
            contributors_sort_alphabetically: None,
            badge_template: None,
            contributor_template: None,
            types: None,
            project_name: project_name,
            project_owner: project_owner,
            repo_type: repo_type,
            repo_host: repo_host,
            link_to_usage: Some(link_to_usage),
            skip_ci: Some(true),
        },
        contributor_file: contributor_file,
        badge_file: "Unimplemented".to_string(),
    }
}
