use anyhow::Result;
use datafusion::prelude::SessionContext;
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::common::file_options::parquet_writer::ParquetWriterOptions;

use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = SessionContext::new();

    let df = ctx.sql(r#"
        SELECT * FROM (
          VALUES
            ('2025-09-01','EU', 120),
            ('2025-09-01','US',  80),
            ('2025-09-02','EU',  50)
        ) AS t(exec_date, site, revenue)
    "#).await?;

    // Partitioned write (Hive-style dirs)
    let write_opts = DataFrameWriteOptions {
        partition_by: vec!["exec_date".into(), "site".into()],
        ..Default::default()
    };

    // Build writer properties (compression, etc.)
    let writer_props: WriterProperties = WriterProperties::builder()
        .set_compression(Compression::ZSTD)
        .build();

    let parquet_opts = ParquetWriterOptions::new(writer_props);

    df.write_parquet("./hive_out", write_opts, Some(parquet_opts)).await?;

    Ok(())
}