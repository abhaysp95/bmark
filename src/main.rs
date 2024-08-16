use std::path::PathBuf;

use anyhow::Result;
use bmark_rs::{is_setup_done, BMark};

mod cli;
mod date;

fn main() -> Result<()> {
    let matches = cli::build_args();

    match matches.subcommand() {
        Some(("setup", setup_task)) => {
            let dbpath = setup_task.get_one::<PathBuf>("dbpath");
            if is_setup_done()? {
                println!("Setup is already done.");
            } else {
                let bmark = BMark::new(dbpath.unwrap().to_owned(), true)?;
                bmark.setup()?;
                println!("Setup completed successfully!!!");
            }
        }
        Some(("add", add_task)) => {
            // check here if the app is setup, by checking dbpath
            if is_setup_done()? {
                let mut bmark = BMark::new("./local/bmark/bmark.db", false)?; // probably provide a config where custom dbpath can be stored on setup, or give option in add for dbpath also
                let url = add_task
                    .get_one::<String>("url")
                    .expect("Providing URL is must");
                let name = add_task
                    .get_one::<String>("name");
                let tags = add_task
                    .get_many::<String>("tags")
                    .unwrap_or_default()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>();
                let desc = add_task.get_one::<String>("description");
                let category = add_task.get_one::<String>("category");
                bmark.insert(
                    &url,
                    name.map(|s| s.as_str()),
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
