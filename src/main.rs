
use clap::Parser;
use anyhow::Result;

// 导入 Opts、SubCommand 和 process_csv 函数
use rcli::{Opts, SubCommand, process_csv};

fn main() -> Result<()> {
    // 解析命令行参数
    let opts: Opts = Opts::parse();
    
    // 根据子命令调用相应的处理函数
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output),
    }
}