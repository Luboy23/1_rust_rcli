
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

/// 表示 CSV 文件中球员的结构体
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,  
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

/// 处理 CSV 文件并将其转换为 JSON 文件的函数
pub fn process_csv(input: &str, output: &str) -> Result<()> {
    // 创建 CSV 读取器
    let mut reader = Reader::from_path(input)?;

    // 用于存储 CSV 记录的 Vec
    let mut ret = Vec::with_capacity(128);

    // 逐行读取 CSV 数据
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }

    // 将 Vec 转换为 JSON 字符串
    let json = serde_json::to_string_pretty(&ret)?;

    // 将 JSON 字符串写入输出文件
    fs::write(output, json)?;
    Ok(())
}
