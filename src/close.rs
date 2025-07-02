use crate::Task;
use dialoguer::Input;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CloseReason {
    Merged,          // 任务被合并
    Duplicate,       // 重复任务
    Obsolete,        // 需求已过时
    NotReproducible, // 无法复现的问题
    WontFix,         // 决定不修复
    Invalid,         // 无效任务/需求
    Other,           // 其他原因
}

impl fmt::Display for CloseReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CloseReason::*;
        match self {
            Merged => write!(f, "任务被合并"),
            Duplicate => write!(f, "重复任务"),
            Obsolete => write!(f, "需求已过时"),
            NotReproducible => write!(f, "无法复现的问题"),
            WontFix => write!(f, "决定不修复"),
            Invalid => write!(f, "无效任务/需求"),
            Other => write!(f, "其他原因"),
        }
    }
}

fn show_reason_menu() {
    println!(
        "选择关闭原因:\n\
         1. 任务被合并\n\
         2. 重复任务\n\
         3. 需求已过时\n\
         4. 无法复现的问题\n\
         5. 决定不修复\n\
         6. 无效任务/需求\n\
         7. 其它"
    );
}

fn close_reason(task: &mut Task, reason: CloseReason) {
    println!("====== 处理任务 {} =====", task.id);
    task.close = Some(reason);
    println!("任务 {} 已关闭 - 原因: {}", task.id, reason);
    println!();
}

pub fn close(task: &mut Task) {
    loop {
        let selection = Input::<String>::new()
            .with_prompt("请输入关闭原因(1-7，查看选项详情输入h)")
            .interact_text();
        
        match selection {
            Ok(s) => {
                use crate::close::CloseReason::*;
                match s.as_str() {
                    "h" => {
                        show_reason_menu();
                        continue;
                    }
                    "1" => close_reason(task, Merged),
                    "2" => close_reason(task, Duplicate),
                    "3" => {
                        println!("你选择了: {:?}", s);
                    }
                    "4" => {
                        println!("你选择了: {:?}", s);
                    }
                    "5" => {
                        println!("你选择了: {:?}", s);
                    }
                    "6" => {
                        println!("你选择了: {:?}", s);
                    }
                    "7" => {
                        println!("你选择了: {:?}", s);
                    }
                    _ => {
                        println!("{} 不是合法选项，请重新输入！", s);
                        continue;
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
        break;
    }
}
