use crate::Task;
use anyhow::anyhow;
use chrono::Duration;
use std::str::FromStr;

use crate::TaskLevel;
use dialoguer::Input;

const DAYS: u32 = 30;

fn get_input(prompt: &str, required: bool) -> String {
    loop {
        let input = Input::<String>::new()
            .with_prompt(prompt)
            .allow_empty(true)
            .interact_text();

        match input {
            Ok(s) => {
                if required && s.trim().is_empty() {
                    println!("该字段不能为空");
                    continue;
                }

                return s;
            }
            Err(e) => println!("{e}"),
        }
    }
}

fn get_priority(prompt: &str, required: bool) -> TaskLevel {
    loop {
        let input = get_input(prompt, required);

        match parse_level(input) {
            Ok(t) => return t,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
    }
}

fn parse_level(input: String) -> anyhow::Result<TaskLevel> {
    use TaskLevel::*;
    match input.trim() {
        "1" => Ok(Urgent),
        "2" => Ok(High),
        "3" => Ok(Medium),
        "4" => Ok(Low),
        _ => {
            let mut err = String::from("无效的优先级，请输入1-4");
            if input.is_empty() {
                err = String::from("该字段不能为空");
            }
            Err(anyhow!(err))
        }
    }
}

pub fn addtask(tasks: &[Task]) -> Task {
    let id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let mut task = Task::default();

    let title = get_input("任务标题 [必填，回车确认]", true);
    let description = get_input("任务描述 [可选，直接回车跳过]", false);
    let owner = get_input("负责人 [可选，直接回车跳过]", false);
    let level = get_priority(
        "优先级 [必填，回车确认] (1：紧急，2：高，3：中，4：低)",
        true,
    );

    let day_s: String = get_input("预计完成天数 [默认30天]", false);
    let days = if day_s.trim().is_empty() {
        DAYS
    } else {
        u32::from_str(day_s.trim()).unwrap_or(DAYS)
    };
    task.date.due_date = Some(task.date.created_at + Duration::days(days.into()));

    println!();
    Task {
        id,
        title: Some(title),
        description: Some(description),
        owner: Some(owner),
        level,
        date: task.date,
        status: task.status,
        close: task.close,
    }
}
