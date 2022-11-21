#![forbid(unsafe_code)]

use all_contributors_rs::init;
use clap::{arg, command, Arg, Command};

fn main() {
    let mut cmd = command!()
        .subcommand(
            Command::new("generate")
                .about("Generate the list of contributors")
        )
        .subcommand(
            Command::new("add")
                .about("Add a new contributor")
                .arg(arg!([username] "Contributor username"))
                .arg(arg!([contribution] "Comma seperated list of contributions"))
        )
        .subcommand(
            Command::new("init")
                .about("Prepare the project to be used with this tool")
        )
        .subcommand(
            Command::new("check")
                .about("Compare contributors from the repository with the ones credited in .all-contributorsrc")
        );

    let matched = cmd.clone().get_matches();

    if let Some(matching) = matched.subcommand_matches("generate") {}

    if let Some(matching) = matched.subcommand_matches("add") {}

    if let Some(matching) = matched.subcommand_matches("init") {
        init::init();
    }

    if let Some(matching) = matched.subcommand_matches("check") {}

    if (matched.subcommand_matches("generate") == None)
        && (matched.subcommand_matches("add") == None)
        && (matched.subcommand_matches("init") == None)
        && (matched.subcommand_matches("check") == None)
    {
        cmd.print_help();
    }
}
