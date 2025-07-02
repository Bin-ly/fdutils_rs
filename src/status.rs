// use chrono::{Local, DateTime};
// use clap::ValueEnum;

// struct Task {
//     id: u32,                // 任务id
//     title: String,          // 任务名称
//     description: Option<String>,    // 任务描述
//     owner: String,          // 负责人
//     level: TaskLevel,       // 等级
//     date: TaskDate,         // 日期
//     status: TaskStatus,     // 状态
// }
// #[derive(Debug)]
// pub struct TaskDate {
//     created_at: DateTime<Local>,            // 创建时间
//     due_date: Option<DateTime<Local>>,      // 截止时间
//     finished_at: Option<DateTime<Local>>    // 实际完成实际
// }
// #[derive(Debug)]
// pub enum TaskLevel {
//     Urgent,  // 紧急
//     High,    // 高
//     Medium,  // 中等
//     Low,     //一般
// }

// #[derive(Debug, Clone, Copy, ValueEnum)]
// pub enum TaskStatus {
//     Pending,                // "未开始"（默认状态）
//     InProgress,             // "进行中"
//     Completed,              // "已完成"
//     Closed,                 // "已关闭"（关闭原因）
//     Expired,                // "已过期"（自动状态）
// }

// #[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
// pub enum CloseReason {
//     UserCancelled,    // 用户主动取消
//     Merged,           // 任务被合并
//     Duplicate,        // 重复任务
//     Obsolete,         // 需求已过时
//     NotReproducible,  // 无法复现的问题
//     WontFix,          // 决定不修复
//     Invalid,          // 无效任务/需求
//     DependencyFailed, // 依赖项失败
//     Other,            // 其他原因
// }
