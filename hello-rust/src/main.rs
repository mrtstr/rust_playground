use clap::{Parser, ValueEnum};
use polars::prelude::{DataFrame, PolarsResult, ParquetCompression, ParquetReader, ParquetWriter, SerReader};
use std::fs::File;
use std::path::Path;
use polars::prelude::*;
use rayon::prelude::*;
use color_eyre::eyre::{WrapErr, Result};

/// A minimal calculator CLI
#[derive(Parser, Debug)]
#[command(name = "compute", version, about = "Example CLI")]
struct Cli {
    /// First operand
    path: String,

    // /// Partition columns (use multiple values)
    // #[arg(long, num_args = 1..)]
    // part_columns: Vec<String>,

    /// Operation to apply
    #[arg(long, value_enum, default_value_t = Op::Read)]
    op: Op,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Op {
    Read,
    Write,
    Process,
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

fn write_parquet(df: &mut DataFrame, path: impl AsRef<Path>) -> color_eyre::eyre::Result<()> {
    let file = File::create(path)?;
    ParquetWriter::new(file)
        .with_compression(ParquetCompression::Snappy)
        .finish(df)?;  // <-- mutable
    Ok(())
}

fn per_partition(df: DataFrame) -> Result<DataFrame> {
    let out = df.lazy()
        .select([
            col("score").sum().alias("score_sum"),
            col("age").sum().alias("age_sum"),
        ])
        .collect()
        .wrap_err("eager select failed")?; // <- returns DataFrame directly
    Ok(out)
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?; // enables pretty error reports
    let args = Cli::parse();

    let p = std::path::Path::new(&args.path);
    if args.op == Op::Read {
        let file = File::open(p)?;
        let reader = std::io::BufReader::new(file);
        let my_df: DataFrame = ParquetReader::new(reader).finish()?;  // memory buffer that can speed things up
        // let my_df: DataFrame = ParquetReader::new(file).finish()?; // no buffer
        println!("read dataframe {my_df}");
    } else if args.op == Op::Process {
        let df = create_df()?;
        let parts: Vec<DataFrame> = df.partition_by(["city"], true)
            .wrap_err("partition_by(city) failed")?;
    
        let processed: Vec<DataFrame> = parts
            .into_par_iter()
            .map(per_partition)
            .collect::<Result<_>>()?;
    
        let out:DataFrame  = polars::functions::concat_df_diagonal(&processed)
            .wrap_err("concat_df failed")?;
        println!("{out}");
    } else {
        let mut my_df = create_df()?;
        write_parquet(&mut my_df, &args.path)?;
        println!("Wrote parquet to {}", p.display());
    }


    println!("Sucessfully executed operation {:?} on path {}", args.op, args.path);
    Ok(())
}

