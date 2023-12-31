// module to handle user inputs from the cli

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]

pub enum Action {
    Add {
        #[structopt()]
        task: String },
    Done {
        #[structopt()]
        position: usize },
    List,
}

pub struct CommandLineArgs {
    pub action: Action,
    pub journal_file: Option<PathBuf>,
}