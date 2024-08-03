use clap::ArgMatches;

mod cli;

fn perform_add_task(matches: &ArgMatches) {
    let url = matches.get_one::<String>("url").expect("Passing URL is must");
    println!("url passed: {}\n", url);
}

fn perform_list_task(_: &ArgMatches) {
    println!("do list");
}

fn main() {
    let matches = cli::build_args();

    if let Some(task) = matches.subcommand_matches("add") {
        perform_add_task(task);
    } else if let Some(task) = matches.subcommand_matches("list") {
        perform_list_task(task);
    }
}
