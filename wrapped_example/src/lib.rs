use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;

use polars::prelude::*;
use polars::prelude::{col, lit, IntoLazy};

#[pyfunction]
fn df_sum_scores(pydf: PyDataFrame) -> PyResult<PyDataFrame> {
    let df: DataFrame = pydf.0; // take inner frame (avoid Into confusion)

    let out = df
        .lazy()
        .select([col("score").sum().alias("score_sum")])
        .collect()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    Ok(PyDataFrame(out))
}

#[pymodule]
fn rustpy(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(df_sum_scores, m)?)?;
    Ok(())
}