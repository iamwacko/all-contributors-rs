const BADGE_CONTENT: &str = "<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->\n[![All Contributors](https://img.shields.io/badge/all_contributors-0-orange.svg?style=flat-square)](#contributors-)\n<!-- ALL-CONTRIBUTORS-BADGE:END -->\n";
const HEADER_CONTENT: &str = "Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):\n\n";
const FOOTER_CONTENT: &str = "This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!\n";
const CONTRIBUTOR_START: &str =
    "<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->\n";
const CONTRIBUTOR_PRETTIER: &str = "<!-- prettier-ignore-start -->\n";
const CONTRIBUTOR_MDLINT: &str = "<!-- markdownlint-disable -->\n";
const CONTRIBUTOR_MDLINT_RESTORE: &str = "<!-- markdownlint-restore -->\n";
const CONTRIBUTOR_PRETTIER_END: &str = "<!-- prettier-ignore-end -->\n";
const CONTRIBUTOR_END: &str = "<!-- ALL-CONTRIBUTORS-LIST:END -->\n\n";

pub fn append_content() -> String {
    let mut string = String::new();
    string.push_str("\n## Contributors ✨\n\n");
    if true {
        string.push_str(HEADER_CONTENT);
    }
    string.push_str(CONTRIBUTOR_START);
    string.push_str(CONTRIBUTOR_PRETTIER);
    string.push_str(CONTRIBUTOR_MDLINT);
    string.push_str(CONTRIBUTOR_MDLINT_RESTORE);
    string.push_str(CONTRIBUTOR_PRETTIER_END);
    string.push_str(CONTRIBUTOR_END);
    if true {
        string.push_str(FOOTER_CONTENT);
    }
    string
}
