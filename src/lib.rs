use pyo3::prelude::*;
use polars::prelude::*;
use pyo3_polars::PyDataFrame;

#[pyfunction]
fn df_sum_scores(py_df: PyDataFrame) -> PyResult<PyDataFrame> {
    let df: DataFrame = py_df.into();
    let out = df.lazy()
        .select([col("score").sum().alias("score_sum")])
        .collect()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(PyDataFrame(out))
}

#[pymodule]
fn rustpy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(df_sum_scores, m)?)?;
    Ok(())
}