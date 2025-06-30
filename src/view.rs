use core::fmt;
use std::default;

use chrono::{DateTime, Local};
use clap::ValueEnum;
use comfy_table::{ContentArrangement, Table};

use crate::status::*;

#[derive(Debug)]
pub struct Task {
    id: u32,                     // 任务id
    title: Option<String>,       // 任务名称
    description: Option<String>, // 任务描述
    owner: Option<String>,       // 负责人
    level: TaskLevel,            // 等级
    date: TaskDate,              // 日期
    status: TaskStatus,          // 状态
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            title: None,
            description: None,
            owner: None,
            level: TaskLevel::High,
            date: TaskDate {
                created_at: Local::now(),
                due_date: None,
                finished_at: None,
            },
            status: TaskStatus::Closed,
        }
    }
}

#[derive(Debug)]
pub enum TaskLevel {
    Urgent, // 紧急
    High,   // 高
    Medium, // 中等
    Low,    //一般
}

impl fmt::Display for TaskLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskLevel::Urgent => write!(f, "紧急"),
            TaskLevel::High => write!(f, "高"),
            TaskLevel::Medium => write!(f, "中等"),
            TaskLevel::Low => write!(f, "一般"),
        }
    }
}

#[derive(Debug)]
pub struct TaskDate {
    created_at: DateTime<Local>,          // 创建时间
    due_date: Option<DateTime<Local>>,    // 截止时间
    finished_at: Option<DateTime<Local>>, // 实际完成实际
}
impl TaskDate {
    fn format_due_date(&self) -> String {
        self.due_date
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "".to_string())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TaskStatus {
    Pending,    // "未开始"（默认状态）
    InProgress, // "进行中"
    Completed,  // "已完成"
    Closed,     // "已关闭"（关闭原因）
    Expired,    // "已过期"（自动状态）
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TaskStatus::*;
        match self {
            Pending => write!(f, "未开始"),
            InProgress => write!(f, "进行中"),
            Completed => write!(f, "已完成"),
            Closed => write!(f, "已关闭"),
            Expired => write!(f, "已过期"),
        }
    }
}

struct TaskView<'a> {
    task: &'a Task,
}

impl<'a> TaskView<'a> {
    const HEADER: [&'static str; 6] = ["ID", "任务名称", "负责人", "优先级", "状态", "截止日期"];
    fn to_row(&self) -> Vec<String> {
        vec![
            self.task.id.to_string(),
            self.task.title.clone().unwrap_or_default(),
            self.task.owner.clone().unwrap_or_default(),
            self.task.level.to_string(),
            self.task.status.to_string(),
            self.task.date.format_due_date(),
        ]
    }
}

pub fn taskview(tasks: Vec<&Task>) {
    let mut table = Table::new();
    table.set_header(TaskView::HEADER)
    .set_content_arrangement(ContentArrangement::Dynamic);

    for task in tasks {
        let view = TaskView { task };
        table.add_row(view.to_row());
    }

    println!("{}", table);
}
