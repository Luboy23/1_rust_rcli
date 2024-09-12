
use clap::Parser;
use anyhow::{Result};

// 导入 Opts、SubCommand 和 process_csv 函数
use rcli::{process_csv, process_genpass, process_encode, process_decode, Base64SubCommand, Opts, SubCommand};

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
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;

            }
        }
    }
    Ok(())
}


