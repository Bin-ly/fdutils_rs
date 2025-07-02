// use std::str::FromStr;

// use crate::TaskDate;
use crate::TaskLevel;
use crate::TaskStatus;
// use crate::date;
use crate::view::Task;
use chrono::NaiveDate;
// use chrono::TimeZone;
// use chrono::NaiveDateTime;
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct UpdateArgs {
    #[arg(long)]
    id: u32,

    #[arg(long)]
    title: Option<String>,

    #[arg(long)]
    description: Option<String>,

    #[arg(long)]
    owner: Option<String>,

    #[arg(long)]
    level: Option<TaskLevel>,

    #[arg(long)]
    date: Option<String>,

    #[arg(long)]
    status: Option<TaskStatus>,
}

impl UpdateArgs {
    pub fn update(&self, tasks: &mut Vec<Task>) -> Result<(), String> {
        match tasks.into_iter().find(|t| t.id == self.id) {
            Some(task) => {
                if let Some(t) = &self.title {
                    task.title = Some(t.to_string());
                }

                if let Some(d) = &self.description {
                    task.description = Some(d.to_string());
                }

                if let Some(o) = &self.owner {
                    task.owner = Some(o.to_string());
                }

                if let Some(o) = &self.level {
                    task.level = *o;
                }

                //待定
                if let Some(o) = &self.date {
                    let formats = ["%Y/%m/%d", "%Y-%m-%d", "%Y%m%d"];
                    let mut parsed = false;
                    for fmt in formats.iter() {
                        if let Ok(naive_date) = NaiveDate::parse_from_str(o, fmt) {
                            task.date.due_date = Some(naive_date);
                            parsed = true;
                        }
                    }
                    if !parsed {
                        println!("无效的日期格式: '{}'。请使用 YYYY/MM/DD 或 YYYY-MM-DD", o);
                    }
                }

                if let Some(s) = &self.status {
                    task.status = *s;
                }

                Ok(())
            }
            None => Err(format!("找不到ID为 {} 的任务", self.id)),
        }
    }
}
