use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use polars::prelude::*;
use eyre::Result;
use rayon::prelude::*;

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
    let out = df.lazy()
        .select([
            col("score").sum().alias("score_sum"),
            col("age").sum().alias("age_sum"),
        ])
        .collect()?;
    Ok(out)
}

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
    Ok(PyDataFrame(out))
}

#[pymodule]
fn wrapped_example_core(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(df_sum_scores, m)?)?;
    m.add_function(wrap_pyfunction!(group_process, m)?)?;
    m.add_function(wrap_pyfunction!(group_process_gil, m)?)?;
    Ok(())
}