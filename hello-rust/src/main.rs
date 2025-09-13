use clap::{Parser, ValueEnum};
use anyhow::{Result, bail};
use polars::prelude::*;
use std::fs::File;
use std::path::Path;

/// A minimal calculator CLI
#[derive(Parser, Debug)]
#[command(name = "compute", version, about = "Example CLI")]
struct Cli {
    /// First operand
    path: String,

    /// Partition columns (use multiple values)
    #[arg(long, num_args = 1..)]
    part_columns: Vec<String>,

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

fn create_df() -> PolarsResult<DataFrame> {
    let df = df![
        "name"  => &["Alice", "Bob", "Charlie"],
        "age"   => &[25i32, 30, 40],
        "score" => &[88.5f64, 95.0, 79.2],
        "city" => &["bs", "bs", "gi"],
    ]?;
    Ok(df)
}

fn write_parquet(df: &mut DataFrame, path: impl AsRef<Path>) -> Result<()> {
    let file = File::create(path)?;
    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Snappy)
        .finish(df)?;  // <-- mutable
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let mut df = create_df()?;
    write_parquet(&mut df, &args.path)?;


    println!("Sucessfully saved the following df und {} \n{df}", args.path);
    // println!("{}, {:?}", args.path, args.part_columns);
    Ok(())
}