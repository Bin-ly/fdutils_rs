use clap::Parser;

mod addtask;
mod close;
mod command;
mod date;
mod list;
mod status;
mod update;
mod view;

use crate::command::{Command, Todo};

use crate::addtask::addtask;
use crate::view::*;

fn main() {
    let todo = Todo::parse();

    if let Err(e) = todo.execute() {
        println!("{e}");
    }
}
