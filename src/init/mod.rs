use inquire::{error::InquireError, Confirm, Select, Text};
use std::fs;
use std::fs::canonicalize;
use std::fs::File;
use std::path::PathBuf;

#[derive(Clone)]
pub struct PromptConfig {
    pub project_name: String,
    pub project_owner: String,
    pub repo_type: String,
    pub repo_host: RepoHost,
    pub contributor_file: String,
    pub need_badge: Option<String>,
    pub image_size: String,
}

#[derive(Clone)]
pub enum RepoHost {
    Gitlab,
    Github,
    Otherhost(String),
}

pub fn init() {
    let config = prompt();
    ensure_file_exists(PathBuf::from(config.clone().contributor_file));
    if let Some(badge) = config.clone().need_badge {}
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
    let list_of_repo_types: Vec<&str> = vec!["GitHub", "GitLab"];

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
                if a == "".to_string() {
                    RepoHost::Otherhost(a)
                } else if repo_type == "GitLab".to_string() {
                    RepoHost::Gitlab
                } else {
                    RepoHost::Github
                }
            }
            Err(_) => panic!("Error processing repo_host"),
        };

    let badge = match Confirm::new("Do you want a badge tallying the number of contributors?")
        .with_default(false)
        .prompt()
    {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => panic!("Error processing badge"),
    };

    let mut need_badge: Option<String> = None;
    if badge {
        let badge_file = match Text::new("In which file should the badge be shown?").prompt() {
            Ok(a) => a,
            Err(_) => panic!("Error processing badge_file"),
        };
        need_badge = Some(badge_file);
    }

    let contributor_file = match Text::new("In which file should contributors be listed?")
        .with_default("README.md")
        .prompt()
    {
        Ok(a) => a,
        Err(_) => panic!("Error processing contributing file"),
    };

    let image_size = match Text::new("How big should the avatars be? (in px)").prompt() {
        Ok(a) => a,
        Err(_) => panic!("Error processing image size"),
    };

    PromptConfig {
        project_name: project_name,
        project_owner: project_owner,
        repo_type: repo_type,
        repo_host: repo_host,
        need_badge: need_badge,
        image_size: image_size,
        contributor_file: contributor_file,
    }
}
