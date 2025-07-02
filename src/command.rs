
use crate::addtask;
use crate::close::close;
use crate::date::load_tasks;
use crate::date::save_tasks;
use crate::list::ListArgs;
use crate::task_detail;
use crate::task_view;
use crate::update::UpdateArgs;
use clap::Parser;
use clap::Subcommand;

// const DATA_PATH: &str = "data/tasks.json";
pub trait Command {
    fn execute(&self) -> anyhow::Result<()>;
}

#[derive(Parser, Debug)]
#[command(version, about = "A simple task management tool")]
pub struct Todo {
    #[arg(short, long)] //查看任务详情
    open: Option<u32>,

    #[arg(short, long)] //添加任务
    add: bool,

    #[command(subcommand)]
    subcommands: Option<SubArgs>,
    
    #[arg(short, long, num_args = 1..)] //关闭任务
    close: Vec<u32>,

    #[arg(short, long)] //删除任务
    delete: Option<u32>,

    #[arg(long)] // 批量关闭所有指定任务(默认原因:完成)
    all: bool,

    #[arg(long)] // 待定，从文件中读取任务
    file: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum SubArgs {
    List(ListArgs),     //筛选任务
    Update(UpdateArgs), // 更新任务
}

impl Command for Todo {
    fn execute(&self) -> anyhow::Result<()> {
        let mut tasks = load_tasks()?;
        if let Some(id) = &self.open {
            if tasks.is_empty() {
                println!("任务列表为空！");
            }
            task_detail(&tasks, *id)
        }

        if self.add {
            let result = addtask(&tasks);
            let id = result.id;

            tasks.push(result);
            match save_tasks(&tasks) {
                Ok(_) => println!("------------任务添加成功！任务ID为：{}------------", id),
                Err(e) => println!("{e}"),
            };
        }

        if !self.close.is_empty() {
            for id in &self.close {
                match tasks.iter_mut().find(|t| t.id == *id) {
                    Some(task) => {
                        close(task);
                    }
                    None => println!("警告: 跳过无效ID {}", id),
                }
            }
            let _ = save_tasks(&tasks);
        }

        if let Some(subcommand) = &self.subcommands {
            use crate::command::SubArgs::*;

            match subcommand {
                List(listargs) => listargs.filter(tasks),
                Update(args) => match args.update(&mut tasks) {
                    Ok(_) => {
                        println!("任务更新成功！");
                        let _ = save_tasks(&tasks);
                        task_view(&tasks);
                    }
                    Err(e) => eprintln!("{}", e),
                },
            }
        }

        Ok(())
    }
}
