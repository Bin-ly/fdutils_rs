use crate::list::ListSubcommand;
use clap::Parser;

pub trait Command {
    fn execute(&self);
}

#[derive(Parser, Debug)]
#[command(version, about = "A simple task management tool")]
pub struct Todo {
    #[arg(short, long)]
    open: Vec<u32>,
    #[arg(short, long)]
    add: bool,
    #[command(subcommand)]
    list: ListSubcommand,
    #[arg(short, long)]
    finish: Vec<u32>,
    #[arg(short, long)]
    close: Vec<u32>,
    #[arg(short, long)]
    delete: Option<u32>,
    #[arg(long)]
    file: Option<String>,
}

impl Command for Todo {
    fn execute(&self) {
        self.list.execute();
    }
}
