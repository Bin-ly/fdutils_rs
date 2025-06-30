use core::task;
use std::io::Take;

use clap::Parser;

mod command;
mod list;
mod status;
mod view;

use crate::command::{Command, Todo};

use crate::view::*;

fn main() {
    // let todo = Todo::parse();
    // println!("{:?}", todo);
    // todo.execute();
    
    let task = Task::default();
    let ta = vec![&task];
    taskview(ta);
}
