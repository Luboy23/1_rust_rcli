use clap::Parser;
use anyhow::Result;


// rcli csx -i input.csv -0 output.json --header -d ','
use rcli::{Opts, SubCommand, process_csv};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    
    match opts.cmd{
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output),
    }
    }
