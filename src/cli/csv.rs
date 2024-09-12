use clap::Parser;
use std::{fmt,str::FromStr};
use super::verify_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

/// 处理 CSV 的选项
#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// 输入文件路径，短选项 -i，长选项 --input，使用 verify_input_file 函数验证文件存在
    #[arg(short, long, value_parser = verify_file)]
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