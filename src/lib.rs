use std::path::PathBuf;

pub enum BMarkTask {
    Add {
        url: String,
        tags: Vec<String>,
        desc: Option<String>,
        category: Option<PathBuf>,
    },
    List {
        output: OutputType,
        cols: ListColumn,
    },
}

pub enum OutputType {
    All(bool),
    Tag(Vec<String>),
}

pub enum ListColumn {
    All,
    Url,
    Tag,
    Desc,
}
