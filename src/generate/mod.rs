use std::fs::canonicalize;
use std::path::PathBuf;

const LOGO_URL: &str = "https://raw.githubusercontent.com/all-contributors/all-contributors-cli/1b8533af435da9854653492b1327a23a4dbd0a10/assets/logo-small.svg";

const LINK_TO_BOT: &str = "https://all-contributors.js.org/docs/en/bot/usage";

pub fn generate() {
    let config = crate::util::config();
    for file in config.clone().files {
        let file_path = canonicalize(&PathBuf::from(file)).expect("Failed to canonicalize files");
        let new_content = generate_content(config.clone().contributors, config.clone().contributors_per_line, config.clone().link_to_usage);
    }
}

fn generate_content(contributors: Vec<crate::Contributor>, per_line: i64, link: Option<bool>) -> String {
    let contributors_list: String = match contributors.len() {
        0 => "\n".to_string(),
        _ => generate_contributors_list(contributors, per_line, link),
    };
    contributors_list
}

fn generate_contributors_list(contributors: Vec<crate::Contributor>, per_line: i64, link: Option<bool>) -> String {
    let footer_content = format_footer(link, per_line);
    let new_content: String = contributors.into_iter().map(|x| format_contributor(x)).collect();
    format!("\n<table>\n  <tbody>\n  <tr>\n    {}    </tr>\n  </tbod>\n{}</table>\n\n", new_content, footer_content)
}

fn format_footer(link: Option<bool>, per_line: i64) -> String {
    if link.is_none() {
        return "".to_string();
    } else if !link.unwrap() {
        return "".to_string();
    } else {
        return format!("<tfoot>\n <tr>\n <td align=\"center\" size = \"13px\" colspan=\"{}\">\n <img src=\"{}\">\n  <a href=\"{}\">Add your contributions</a>\n </img>\n </td>\n</tfoot>\n", per_line, LOGO_URL, LINK_TO_BOT);
    }
}

fn format_contributor(contributor: crate::Contributor) -> String {
    "".to_string()
}
