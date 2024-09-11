use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

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
    GenPass(GenPassOpts)

}



#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

/// 处理 CSV 的选项
#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// 输入文件路径，短选项 -i，长选项 --input，使用 verify_input_file 函数验证文件存在
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// 输出文件路径，短选项 -o，长选项 --output，默认值为 "output.json"
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg( long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,

    /// CSV 文件的分隔符，短选项 -d，长选项 --delimiter，默认值为 ','
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// 是否包含 CSV 文件的表头，长选项 --header，默认值为 true
    #[arg(long, default_value_t = true)]
    pub header: bool,
}
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,  
}

/// 验证输入文件是否存在的函数
fn verify_input_file(file_name: &str) -> Result<String, String> {
    if Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("文件不存在!".into())
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format{
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v =>anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}