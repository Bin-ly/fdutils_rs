use crate::TaskLevel;
use crate::task_view;
use crate::view::Task;
use crate::view::TaskStatus;
use clap::Args;

// #[derive(Subcommand, Debug)]
// pub enum SubArgs {
//     List(ListArgs),
//     Update(ListArgs),
// }

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(long, num_args = 1..)] // 接受 1 个或多个值
    id: Option<Vec<u32>>,

    #[arg(long)]
    status: Option<TaskStatus>,

    #[arg(long)]
    owner: Option<String>,

    #[arg(long)]
    level: Option<TaskLevel>,
}

impl ListArgs {
    pub fn filter(&self, tasks: Vec<Task>) {
        let mut filtered: Vec<Task> = tasks.into_iter().collect();
        if let Some(i) = &self.id {
            filtered.retain(|t| i.contains(&t.id));
        }

        if let Some(s) = self.status {
            filtered.retain(|t| t.status == s);
        }

        if let Some(l) = self.level {
            filtered.retain(|t| t.level == l);
        }

        if let Some(o) = &self.owner {
            filtered.retain(|t| t.owner == Some(o.clone()));
        }

        task_view(&filtered);
    }
}
