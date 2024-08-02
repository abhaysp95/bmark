use clap::{Arg, ArgAction, ArgGroup, Command};

// NOTE: what should be global
// filepath (dbpath)

// #[derive(Parser)]
// #[command(about = "Bookmark Manager tool", long_about = None)]
// struct BMark {
//     #[command(subcommand)]
//     command: Commands,
//     path: String, // NOTE: Looks like this can't be a path
// }
//
// #[derive(Subcommand)]
// enum Commands {
//     Add(AddBmark),
//     List(ListBmark),
// }
//
// #[derive(Args)]
// struct AddBmark {
//     #[arg(short, long)]
//     url: String,
//     // NOTE: should I keep it comma seperated or make it multi ?
//     #[arg(short, long)]
//     tags: Vec<String>,
//     #[arg(short, long)]
//     desc: Option<String>,
//     // NOTE: or should it be path... if String, it will be dot(.) separated
//     #[arg(short, long, default_value_t = String::from("root"))]
//     category: String,
// }
//
// #[derive(Args)]
// struct ListBmark {
//     #[arg(short, long)]
//     category: Option<String>,
//     #[arg(short, long, group = "list-type")]
//     output: bool,
//     #[arg(short, long, group = "list-columns")]
//     cols: bool,
// }

fn build_args() {
    let b = Command::new("bmark")
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
                    .required(false)
                    .help("Provide tags to bookmark"),
                Arg::new("description")
                    .short('d')
                    .long("desc")
                    .required(false)
                    .help("Additional note for bookmark"),
                Arg::new("category")
                    .short('c')
                    .long("catg")
                    .required(false)
                    .help("Category to put the URL in [default: root]"),
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
                .group(ArgGroup::new("output").args(["all", "tag"]).required(true)),
        )
        .get_matches();
}

fn main() {
    build_args();
}

// #[derive(Args)]
// #[group(required = true, multiple = false)]
// struct ListType {
//     #[arg(short, long)]
//     full: bool,
//     #[arg(short, long)]
//     tags: bool,
// }
//
// #[derive(Args)]
// #[group(required = true, multiple = false)]
// struct ListColumns {
//     #[arg(short, long)]
//     all: bool,
//     #[arg(short, long)]
//     url: bool,
//     #[arg(short, long)]
//     desc: bool,
//     #[arg(short, long)]
//     tags: bool,
// }
