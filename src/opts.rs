use clap::Parser;
use std::path::Path;

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
}

/// 处理 CSV 的选项
#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// 输入文件路径，短选项 -i，长选项 --input，使用 verify_input_file 函数验证文件存在
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// 输出文件路径，短选项 -o，长选项 --output，默认值为 "output.json"
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    /// CSV 文件的分隔符，短选项 -d，长选项 --delimiter，默认值为 ','
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// 是否包含 CSV 文件的表头，长选项 --header，默认值为 true
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/// 验证输入文件是否存在的函数
fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("文件不存在!".into())
    }
}