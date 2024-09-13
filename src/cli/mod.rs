mod csv;
mod genpass;
mod base64;
mod text;

use clap::Parser;
use std::path::PathBuf;

pub use self::{csv::CsvOpts, genpass::GenPassOpts};
use std::path::Path;
pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    text::{TextSignFormat,TextSubCommand}
};

/// 解析命令行参数的结构体
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    /// 子命令，可能是处理 CSV 的子命令
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// 子命令的枚举，包含处理 CSV 的选项
#[derive(Debug, Parser)]
pub enum SubCommand {
    /// 处理 CSV 文件的子命令
    #[command(name = "csv", about = "处理 CSV 文件或将 CSV 转换为其他格式")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "生成一个随机密码")]
    GenPass(GenPassOpts),

    #[command(subcommand)]
    Base64(Base64SubCommand),

    #[command(subcommand)]
    Text(TextSubCommand),
}

/// 验证输入文件是否存在的函数
fn verify_file(file_name: &str) -> Result<String, &'static str> {
    if file_name == "-" || Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("文件不存在!".into())
    } 
}
fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input() {
        assert_eq!(verify_file("-"),Ok("-".into()));
        assert_eq!(verify_file("*"), Err("文件不存在!"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("no-exist"), Err("文件不存在!"));
    }
}







