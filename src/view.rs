use core::fmt;
use serde::{Deserialize, Serialize};
use crate::close::CloseReason;
use chrono::{Local, NaiveDate};
use clap::ValueEnum;
use comfy_table::{ContentArrangement, Table};


#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,                     // 任务id
    pub title: Option<String>,       // 任务名称
    pub description: Option<String>, // 任务描述
    pub owner: Option<String>,       // 负责人
    pub level: TaskLevel,            // 等级
    pub date: TaskDate,              // 日期
    pub status: TaskStatus,          // 状态
    pub close: Option<CloseReason>,          // 状态
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            title: None,
            description: None,
            owner: None,
            level: TaskLevel::Low,
            date: TaskDate {
                created_at: Local::now().date_naive(),
                due_date: None,
                finished_at: None,
            },
            status: TaskStatus::Pending,
            close: None,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDate {
    pub created_at: NaiveDate,          // 创建时间
    pub due_date: Option<NaiveDate>,    // 截止时间
    pub finished_at: Option<NaiveDate>, // 实际完成实际
}

#[derive(Debug, Clone, Copy, ValueEnum, Serialize, Deserialize, PartialEq)]
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
    const HEADER: [&'static str; 6] = ["ID", "任务名称", "负责人", "优先级", "状态", "创建日期"];
    fn to_row(&self) -> Vec<String> {
        vec![
            self.task.id.to_string(),
            self.task.title.clone().unwrap_or_default(),
            self.task.owner.clone().unwrap_or_default(),
            self.task.level.to_string(),
            self.task.status.to_string(),
            self.task.date.created_at.to_string(),
        ]
    }
}

pub fn task_view(tasks: &Vec<Task>) {
    let mut table = Table::new();
    table
        .set_header(TaskView::HEADER)
        .set_content_arrangement(ContentArrangement::Dynamic);

    if tasks.is_empty() {
        println!("{}", table);
        println!("没有任务");
        return;
    }

    for task in tasks {
        let view = TaskView { task };
        table.add_row(view.to_row());
    }

    println!("{}", table);
}

pub fn task_detail(tasks: &[Task], id: u32) {
    if let Some(task) = tasks.iter().find(|t| t.id == id) {
        // 计算剩余天数
        let days = task.date.due_date.map_or(0, |due_date| {
            (due_date - Local::now().date_naive()).num_days().max(0)
        });

        let mut table = Table::new();

        table
            .set_content_arrangement(ContentArrangement::Dynamic)
            .load_preset(comfy_table::presets::NOTHING)
            .set_width(80);

        println!(
            "[任务#{}] {}",
            task.id,
            task.title.as_deref().unwrap_or("无标题")
        );
        println!("{:-<30}", ""); // 分隔线

        let mut  fields = vec![
            ("状态", format!("{} (剩余{}天)", task.status, days)),
            ("负责人", task.owner.clone().unwrap_or_default()),
            ("优先级", task.level.to_string()),
            ("创建日期", task.date.created_at.to_string()),
            (
                "截止日期",
                task.date
                    .due_date
                    .map(|d| d.to_string())
                    .unwrap_or_default(),
            ),
            ("描述", task.description.clone().unwrap_or_default()),
        ];
        
        if task.status == TaskStatus::Closed {
            fields.push((
                    "关闭原因", 
                    task.close.as_ref()
                        .map(|r| r.to_string())  // 这里会自动使用 Display 实现
                        .unwrap_or("未指定原因".to_string())
            ));
        }

        for (name, value) in fields {
            table.add_row(vec![format!("{}:", name), value]);
        }
        println!("{}", table);
    } else {
        println!("未找到ID为 {} 的任务", id);
    }
}
