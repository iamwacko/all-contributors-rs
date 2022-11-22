use serde::{Deserialize, Serialize};

pub mod init;
pub mod generate;
pub mod add;
pub mod check;
pub mod util;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub files: Vec<String>,
    pub image_size: i64,
    pub commit: bool,
    pub commit_convention: String,
    pub contributors: Vec<Contributor>,
    pub contributors_per_line: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors_sort_alphabetically: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributor_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Types>,
    pub project_name: String,
    pub project_owner: String,
    pub repo_type: String,
    pub repo_host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_to_usage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_ci: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub profile: String,
    pub contributions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Types {
    pub custom: Custom,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub symbol: String,
    pub description: String,
    pub link: String,
}
