use crate::command::Command;
use crate::view::TaskStatus;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum ListSubcommand {
    List(ListArgs),
}

impl Command for ListSubcommand {
    fn execute(&self) {
        use ListSubcommand::*;
        match self {
            List(c) => c.execute(),
        }
    }
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(short, long)]
    status: Option<TaskStatus>,
    #[arg(short, long)]
    owner: Option<String>,
    #[arg(short, long)]
    priority: Option<String>,
}

impl Command for ListArgs {
    fn execute(&self) {
        println!("--------------");
        println!("{:?}", self);
    }
}
