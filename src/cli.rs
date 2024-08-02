use std::path::PathBuf;

use clap::{value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};

pub fn build_args() -> ArgMatches {
    return Command::new("bmark")
        // .no_binary_name(true)
        .author("abhay")
        .version("0.0.1") // will make dynamic
        .about("Bookmark Manager tool")
        .subcommand(
            Command::new("add").args([
                Arg::new("url")
                    .short('u')
                    .long("url")
                    .required(true)
                    .help("URL to bookmark"),
                Arg::new("tags")
                    .short('t')
                    .long("tag")
                    .action(ArgAction::Append)
                    .help("Provide tags to bookmark"),
                Arg::new("description")
                    .short('d')
                    .long("desc")
                    .help("Additional note for bookmark"),
                Arg::new("category")
                    .short('c')
                    .long("catg")
                    .default_value("root")
                    .value_parser(value_parser!(PathBuf))
                    .help("Category to put the URL in"),
            ]),
        )
        .subcommand(
            Command::new("list")
                .args([
                    Arg::new("all")
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .help("List out all for bookmark"),
                    // can't this be multiple yet "all" be single ?
                    Arg::new("tag")
                        .short('t')
                        .long("tag")
                        .action(ArgAction::Append)
                        .help("List out bookmarks related to tag"),
                ])
                .group(ArgGroup::new("output").args(["all", "tag"]).required(true))
                .arg(
                    Arg::new("cols")
                        .short('c')
                        .long("cols")
                        .default_value("all")
                        .value_parser(["all, url, desc, tags"])
                        .help("List the specified cols")
                        .long_help(
                            "List the specified columns.

Either provide \"all\" or provide any of the others mentioned columns.
\"url\" will just list out the URLs without any other columns.
If provided \"desc\" or \"tags\" as value, they will be listed along with their URL.",
                        ),
                ),
        )
        .get_matches();
}


