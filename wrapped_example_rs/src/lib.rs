use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use polars::prelude::*;
use eyre::Result;
use rayon::prelude::*;
use std::sync::Once;
use log::{info, warn, error};

static INIT: Once = Once::new();

fn init_logging() {
    INIT.call_once(|| {
        env_logger::Builder::from_env(
            env_logger::Env::default().default_filter_or("info")  // default = info
        )
        .format_timestamp_millis()
        .init();
    });
}

#[pyfunction]
#[pyo3(signature = (pydf, col_name="score", out_name=None))]
fn df_sum_scores(pydf: PyDataFrame, col_name: &str, out_name: Option<&str>) -> PyResult<PyDataFrame> {
    // Inner block returns eyre::Result<_>; `?` works everywhere.
    let out = (|| -> Result<DataFrame> {
        let col = pydf.0
            .column(col_name)?
            // .map_err(|e| eyre!("missing 'score' column: {e}"))? // add manually context
            .cast(&DataType::Float64)?;

        let sum: f64 = col.f64()?.sum().unwrap_or(0.0);
        let out_name_owned = out_name.map(str::to_string).unwrap_or_else(|| format!("{col_name}_sum"));
        let out = df!(out_name_owned => [sum])?;
        Ok(out)
    })()?;
    // add rust stack trace with the following
    // })().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("{e:#}\n\n[Rust backtrace]\n{}", std::backtrace::Backtrace::capture())))?; 
    Ok(PyDataFrame(out))
}

fn per_partition(df: DataFrame) -> Result<DataFrame> {
    // --- age ---
    let age_col = df.column("age")?
        .cast(&DataType::Float64)?; // ensure consistent dtype
    let age_ca = age_col.f64()?;    // ChunkedArray<Float64>

    // Access the Arrow chunks as slices
    let age_sum: f64 = age_ca.downcast_iter()
        .flat_map(|arr| arr.values().as_slice())
        .copied()
        .sum();

    // --- score ---
    let score_col = df.column("score")?
        .cast(&DataType::Float64)?;
    let score_ca = score_col.f64()?;

    let score_sum: f64 = score_ca.downcast_iter()
        .flat_map(|arr| arr.values().as_slice())
        .copied()
        .sum();

    // Wrap back into a one-row DataFrame
    let out = df!(
        "age" => [age_sum],
        "score" => [score_sum],
    )?;
    Ok(out)
}

// fn per_partition(df: DataFrame) -> Result<DataFrame> {

//     let age_col = df
//         .column("age")?
//         // .map_err(|e| eyre!("missing 'score' column: {e}"))? // add manually context
//         .cast(&DataType::Float64)?;
//     let score_col = df
//         .column("score")?
//         // .map_err(|e| eyre!("missing 'score' column: {e}"))? // add manually context
//         .cast(&DataType::Float64)?;

//     let age_sum: f64 = age_col.f64()?.sum().unwrap_or(0.0);
//     let score_sum: f64 = score_col.f64()?.sum().unwrap_or(0.0);


//     let out = df!(
//         "age" => [age_sum],
//         "score" => [score_sum],
//     )?;
//     Ok(out)
// }

// fn per_partition(df: DataFrame) -> Result<DataFrame> {
//     let out = df.lazy()
//         .select([
//             col("score").sum().alias("score_sum"),
//             col("age").sum().alias("age_sum"),
//         ])
//         .collect()?;
//     Ok(out)
// }

#[pyfunction]
#[pyo3(signature = (pydf, part_col_name))]
fn group_process(py: Python<'_>, pydf: PyDataFrame, part_col_name: &str) -> PyResult<PyDataFrame> {
    let df = pydf.0;
    let out = py.allow_threads(move || -> Result<DataFrame> {
        let parts: Vec<DataFrame> = df.partition_by([part_col_name], true)?;

        let processed: Vec<DataFrame> = parts
            .into_par_iter()
            .map(per_partition)
            .collect::<Result<_>>()?;

        let out:DataFrame  = polars::functions::concat_df_diagonal(&processed)?;
        Ok(out)
    })?;
    Ok(PyDataFrame(out))
}

#[pyfunction]
#[pyo3(signature = (pydf, part_col_name))]
fn group_process_gil(pydf: PyDataFrame, part_col_name: &str) -> PyResult<PyDataFrame> {
    let df = pydf.0;
    let out = (|| -> Result<DataFrame> {
        let parts: Vec<DataFrame> = df.partition_by([part_col_name], true)?;

        let processed: Vec<DataFrame> = parts
            .into_par_iter()
            .map(per_partition)
            .collect::<Result<_>>()?;

        let out:DataFrame  = polars::functions::concat_df_diagonal(&processed)?;
        Ok(out)
    })()?;
    info!("All parts processed");
    Ok(PyDataFrame(out))
}

#[pymodule]
fn wrapped_example_core(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    init_logging();

    m.add_function(wrap_pyfunction!(df_sum_scores, m)?)?;
    m.add_function(wrap_pyfunction!(group_process, m)?)?;
    m.add_function(wrap_pyfunction!(group_process_gil, m)?)?;
    info!("wrapped_example_core module initialized");
    Ok(())
}