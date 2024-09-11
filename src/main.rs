
use clap::Parser;
use anyhow::{Result};

// 导入 Opts、SubCommand 和 process_csv 函数
use rcli::{Opts, SubCommand, process_csv, process_genpass};

fn main() -> Result<()> {
    // 解析命令行参数
    let opts: Opts = Opts::parse();
    
    // 根据子命令调用相应的处理函数
    match opts.cmd {
        SubCommand::Csv(opts) =>{
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbol)?;
        }
    }
    Ok(())
}