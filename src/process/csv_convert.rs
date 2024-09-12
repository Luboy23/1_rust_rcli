
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

use crate::cli::OutputFormat;

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
pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // 创建 CSV 读取器
    let mut reader = Reader::from_path(input)?;

    // 用于存储 CSV 记录的 Vec
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();

    // 逐行读取 CSV 数据
    for result in reader.records(){
        let record = result?;

        // headers.iter() -> 使用 headers 的迭代器
        // record.iter() -> 使用 record 的迭代器
        // zip() 将两个迭代器合并为一个元组的迭代器[(header, record), ..]
        // collect::<Value> -> 将元组的迭代器转换为 JSON Value
        let json_value = headers.iter().zip(record.iter()).collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    // 将 JSON 字符串写入输出文件
    fs::write(output, content)?;
    Ok(())
}
