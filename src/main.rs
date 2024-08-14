use std::path::PathBuf;

use anyhow::Result;
use bmark_rs::{is_setup_done, BMark};
use clap::ArgMatches;

mod cli;
mod date;

fn perform_add_task(matches: &ArgMatches) -> Result<()> {
    let url = matches
        .get_one::<String>("url")
        .expect("Passing URL is must");
    // let conn = get_db_connection(None)?;
    // create_table(&conn, "new_table")?;
    println!("url passed: {}\n", url);

    date::validate_date("").expect("Some date");

    Ok(())
}

fn main() -> Result<()> {
    let matches = cli::build_args();

    match matches.subcommand() {
        Some(("setup", setup_task)) => {
            let dbpath = setup_task.get_one::<PathBuf>("dbpath");
            let bmark = BMark::new(dbpath.unwrap().to_owned())?;
            bmark.setup()?;
            println!("Setup completed successfully!!!");
        }
        Some(("add", add_task)) => {
            // check here if the app is setup, by checking dbpath
            if is_setup_done()? {
                let bmark = BMark::new("./local/bmark/bmark.db")?; // probably provide a config where custom dbpath can be stored on setup, or give option in add for dbpath also
                let url = add_task
                    .get_one::<String>("url")
                    .expect("Providing URL is must");
                let tags = add_task
                    .get_many::<String>("tags")
                    .unwrap_or_default()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>();
                let desc = add_task.get_one::<String>("description");
                let category = add_task.get_one::<String>("category");
                bmark.insert(
                    &url,
                    tags,
                    desc.map(|s| s.as_str()),
                    category.map(|s| s.as_str()),
                )?;
            } else {
                println!("You need to do setup first. Run: bmark setup --help for more info");
            }
        }
        Some(("list", list_task)) => {}
        _ => {}
    }

    Ok(())
}
