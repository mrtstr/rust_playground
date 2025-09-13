use clap::{Parser, ValueEnum};
use anyhow::{Result, bail};

/// A minimal calculator CLI
#[derive(Parser, Debug)]
#[command(name = "compute", version, about = "Example CLI")]
struct Cli {
    /// First operand
    x: i64,

    /// Second operand
    y: i64,

    /// Operation to apply
    #[arg(long, value_enum, default_value_t = Op::Add)]
    op: Op,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let res = match args.op {
        Op::Add => args.x + args.y,
        Op::Sub => args.x - args.y,
        Op::Mul => args.x * args.y,
        Op::Div => {
            if args.y == 0 {
                bail!("division by zero");
            }
            args.x / args.y
        }
    };

    println!("{res}");
    Ok(())
}