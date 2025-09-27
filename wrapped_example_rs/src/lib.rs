use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use polars::prelude::*;
use eyre::Result;

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


#[pymodule]
fn wrapped_example_core(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(df_sum_scores, m)?)?;
    Ok(())
}