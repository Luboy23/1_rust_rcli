
use std::fs;

use clap::Parser;
use anyhow::Result;
use zxcvbn::zxcvbn;


// 导入 Opts、SubCommand 和 process_csv 函数
use rcli::{process_csv, process_decode, process_encode, process_generate, process_genpass, process_text_sign, process_text_verify, process_http_serve,
            Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand };

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
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
        },
        SubCommand::GenPass(opts) => {
            let password =  process_genpass(opts.length, opts.uppercase, opts.lowercase, opts.number, opts.symbol)?;
            println!("password: {}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", estimate.score());

        },
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                
                // TODO: decoded data might not be string(but for this example, we assume it is)
                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);

            }
            TextSubCommand::Verify(opts) => {
                let verified =  process_text_verify(&opts.input, &opts.key, opts.format,&opts.sig)?;
                println!("{:?}", verified);

            }
            TextSubCommand::Generate(opts) => {
                let key = process_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
         },
         SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
         }
}
Ok(())
}


