use anyhow::Result;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let root = "./data/sales_ds";

    // 1) WRITE a Hive-partitioned Parquet dataset with DataFusion
    write_partitioned_with_datafusion(root).await?;

    // 2) READ it back with DataFusion (partition columns auto-discovered)
    read_partitioned_with_datafusion(root).await?;

    // 3) READ the Parquet files with Polars (simple example)
    read_with_polars(root)?;

    Ok(())
}

async fn write_partitioned_with_datafusion(root: &str) -> datafusion::error::Result<()> {
    let mut ctx = SessionContext::new();

    // Tiny in-memory table via SQL VALUES (3 rows)
    // Columns: exec_date (string), site (string), revenue (int)
    let df = ctx
        .sql(r#"
            SELECT * FROM (
              VALUES
                ('2025-09-01','EU', 120),
                ('2025-09-01','US',  80),
                ('2025-09-02','EU',  50)
            ) AS t(exec_date, site, revenue)
        "#)
        .await?;

    // Write Hive-style: {root}/exec_date=YYYY-MM-DD/site=XX/part-*.parquet
    use datafusion::datasource::file_writer::FileType;
    use datafusion::execution::options::{WriteOptions, WriteTableOptions};
    use datafusion::datasource::file_format::parquet::ParquetWriterOptions;
    use parquet::basic::Compression;

    let opts = WriteTableOptions {
        file_type: FileType::PARQUET,
        write_options: WriteOptions::default(), // set overwrite if you need
        partition_by: vec!["exec_date".into(), "site".into()],
        parquet_options: Some(
            ParquetWriterOptions::default().with_compression(Compression::ZSTD)
        ),
        ..Default::default()
    };

    df.write_table(root, opts).await?;
    println!("✔ wrote Hive-partitioned Parquet under: {root}");
    Ok(())
}

async fn read_partitioned_with_datafusion(root: &str) -> datafusion::error::Result<()> {
    let mut ctx = SessionContext::new();

    // Register a listing table over the root; DF infers partition columns from dir names
    use datafusion::datasource::listing::ListingOptions;
    use datafusion::datasource::file_format::parquet::ParquetFormat;

    let listing = ListingOptions::new(ParquetFormat::default())
        .with_file_extension(".parquet")
        .with_collect_stat(true);

    ctx.register_listing_table("sales", root, listing, None, None).await?;

    let df = ctx.sql(r#"
        SELECT exec_date, site, SUM(revenue) AS total_rev
        FROM sales
        GROUP BY exec_date, site
        ORDER BY exec_date, site
    "#).await?;

    let batches = df.collect().await?;
    datafusion::arrow::util::pretty::print_batches(&batches).unwrap();

    Ok(())
}

fn read_with_polars(root: &str) -> polars::prelude::PolarsResult<()> {
    use polars::prelude::*;

    // NOTE: The partition columns (exec_date, site) are in directory names.
    // Depending on writer settings, they may NOT be embedded in the file schema.
    // So here we just show reading the files and doing a simple aggregate.
    let pattern = format!("{root}/**/*.parquet");

    let lf = LazyFrame::scan_parquet(
        &pattern,
        ScanArgsParquet::default(),    // simple defaults
    )?;

    // If the files contain 'revenue' column, we can aggregate:
    let out = lf.select([col("revenue").sum().alias("sum_revenue")]).collect()?;
    println!("{out}");

    Ok(())
}
