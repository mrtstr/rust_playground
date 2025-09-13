// use anyhow::{bail, Context, Result};
// use clap::{ArgAction, Parser, ValueEnum};
// use std::path::PathBuf;

// /// hello-rust: simple CLI without subcommands
// ///
// /// Examples:
// ///   hello-rust --name Alice                # greet (default lang EN)
// ///   hello-rust --name Bob --lang de        # greet in German
// ///   hello-rust --x 6 --y 7                 # compute 6 + 7
// ///   hello-rust --x 8 --y 2 --op div        # compute 8 / 2
// #[derive(Parser, Debug)]
// #[command(name = "hello-rust", version, about = "Example CLI")]
// struct Cli {
//     /// Increase verbosity (-v, -vv, -vvv)
//     #[arg(short, long, action = ArgAction::Count)]
//     verbose: u8,

//     /// Optional config file (can also be set via APP_CONFIG)
//     #[arg(short, long, env = "APP_CONFIG")]
//     config: Option<PathBuf>,

//     // --- Greet mode ---
//     /// Name to greet (selects 'greet' mode if provided)
//     #[arg(long)]
//     name: Option<String>,

//     /// Greeting language
//     #[arg(long, value_enum, default_value_t = Lang::En)]
//     lang: Lang,

//     // --- Compute mode ---
//     /// First operand (selects 'compute' mode if provided; requires --y)
//     #[arg(long)]
//     x: Option<i64>,

//     /// Second operand (requires --x)
//     #[arg(long)]
//     y: Option<i64>,

//     /// Operation to apply
//     #[arg(long, value_enum, default_value_t = Op::Add)]
//     op: Op,
// }

// #[derive(Copy, Clone, Debug, ValueEnum)]
// enum Lang {
//     En,
//     De,
// }

// #[derive(Copy, Clone, Debug, ValueEnum)]
// enum Op {
//     Add,
//     Sub,
//     Mul,
//     Div,
// }

// fn main() -> Result<()> {
//     let args = Cli::parse();

//     if args.verbose > 0 {
//         eprintln!("[debug] verbosity={} config={:?}", args.verbose, args.config);
//     }

//     match (&args.name, args.x, args.y) {
//         // ---- Greet mode ----
//         (Some(name), None, None) => {
//             let msg = match args.lang {
//                 Lang::En => format!("Hello, {}!", name),
//                 Lang::De => format!("Hallo, {}!", name),
//             };
//             println!("{msg}");
//         }

//         // ---- Compute mode ----
//         (None, Some(x), Some(y)) => {
//             let res = match args.op {
//                 Op::Add => x + y,
//                 Op::Sub => x - y,
//                 Op::Mul => x * y,
//                 Op::Div => {
//                     if y == 0 {
//                         bail!("division by zero");
//                     }
//                     x / y
//                 }
//             };
//             println!("{res}");
//         }

//         // ---- Ambiguous or incomplete ----
//         (Some(_), Some(_), _) | (Some(_), _, Some(_)) => {
//             bail!("choose ONE mode: either --name for greeting OR --x/--y for compute");
//         }
//         (None, Some(_), None) | (None, None, Some(_)) => {
//             bail!("compute mode requires BOTH --x and --y");
//         }
//         _ => {
//             // No mode chosen: show help text
//             let mut cmd = Cli::command();
//             bail!(cmd.render_usage().to_string());
//         }
//     }

//     Ok(())
// }