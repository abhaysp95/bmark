use std::path::PathBuf;

use clap::{value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};

pub fn build_args() -> ArgMatches {
    return Command::new("bmark")
        // .no_binary_name(true)
        .author("abhay")
        .version("0.0.1") // will make dynamic
        .about("Bookmark Manager tool")
        .subcommand(
            Command::new("setup").args([
                Arg::new("dbpath")
                    .long("dbpath")
                    .default_value("./local/bmark/bmark.db")
                    .value_parser(value_parser!(PathBuf))
                    .help("Tell where the db file should be placed")
            ])
        )
        .subcommand(
            Command::new("add").args([
                Arg::new("url")
                    .short('u')
                    .long("url")
                    // Will add url validation and stuff once I figure out the lifetime issue here
                    // .value_parser(|s: &str| {
                    //     if s.contains("|") {
                    //         Err(clap::Error::new(clap::error::ErrorKind::ValueValidation))
                    //     } else {
                    //         Ok(s)
                    //     }
                    // })
                    .required(true)
                    .help("URL to bookmark"),
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .help("Name for the URL"),
                Arg::new("tags")
                    .short('t')
                    .long("tag")
                    .action(ArgAction::Append)
                    .help("Provide tags to bookmark [support multiple tags]"),
                Arg::new("description")
                    .long("desc")
                    .help("Additional note for bookmark"),
                Arg::new("category")
                    .short('c')
                    .long("catg")
                    .help("Category to put the URL in"),
                // Arg::new("date")
                //     .long("date")
                //     .value_parser(validate_date)
                //     .help("Date of when the bookmark was added [default: today, format: yyyy-mm-dd]"),
            ]),
        )
        .subcommand(
            Command::new("list")
                .args([
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .help("List out all for bookmark. If --tag/-t not passed, --all will be considered"),
                    // can't this be multiple yet "all" be single ?
                    Arg::new("tag")
                        .short('t')
                        .long("tag")
                        .action(ArgAction::Append)
                        .help("List out bookmarks related to tag [support multiple tags]"),
                ])
                .group(ArgGroup::new("output").args(["all", "tag"]).required(true))
                .arg(
                    Arg::new("cols")
                        .short('c')
                        .long("cols")
                        .default_value("all")
                        .value_parser(["all", "url", "desc", "tags"])
                        .help("List the specified cols")
                        .long_help(
                            "List the specified columns.

Either provide \"all\" or provide any of the others mentioned columns.
\"url\" will just list out the URLs without any other columns.
If provided \"desc\" or \"tags\" as value, they will be listed along with their URL.",
                        ),
                )
                .arg(
                    Arg::new("tag-mode")
                        .default_value("any")
                        .value_parser(["all", "any"])
                        .help("When 'all' enabled it'll strictly look for the bookmarks which have all the tags given by user"))
        )
        .get_matches();
}
