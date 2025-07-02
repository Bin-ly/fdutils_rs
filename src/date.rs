use std::{fs, path::Path};

use crate::Task;
const DATA_PATH: &str = "data/tasks.json";

// 序列化并保存到文件（自动创建或覆盖）
pub fn save_tasks(tasks: &[Task]) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(tasks)?; //自动创建父目录（如果不存在）

    if let Some(parent) = Path::new(DATA_PATH).parent() {
        fs::create_dir_all(parent)?
    }
    fs::write(DATA_PATH, json)?; //自动创建或覆盖文件

    Ok(())
}

// 从文件加载并反序列化（自动处理文件不存在）
pub fn load_tasks() -> anyhow::Result<Vec<Task>> {
    if !Path::new(DATA_PATH).exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(DATA_PATH)?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}
