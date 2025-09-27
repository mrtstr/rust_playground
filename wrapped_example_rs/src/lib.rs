use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use polars::prelude::*;
use eyre::{Result, eyre}; // ergonomic error handling

#[pyfunction]
#[pyo3(signature = (pydf))]
fn df_sum_scores(pydf: PyDataFrame) -> PyResult<PyDataFrame> {
    // Inner block returns eyre::Result<_>; `?` works everywhere.
    let out = (|| -> Result<DataFrame> {
        let col = pydf.0
            .column("score")?
            // .map_err(|e| eyre!("missing 'score' column: {e}"))? // add manually context
            .cast(&DataType::Float64)?;

        let sum: f64 = col.f64()?.sum().unwrap_or(0.0);
        let out = df!("score_sum" => [sum])?;
        Ok(out)
    })()?;
    Ok(PyDataFrame(out))
}

// #[pyfunction]
// #[pyo3(signature = (pydf))]
// fn df_sum_scores(pydf: PyDataFrame) -> PyResult<PyDataFrame> {
//     let df: DataFrame = pydf.0;
//     let sum = df
//         .column("score")
//         .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
//         .cast(&DataType::Float64)
//         .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
//         .f64()
//         .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?
//         .sum().unwrap();

//     let out = DataFrame::new(vec![Column::new("a_sum".into(), &[sum])]).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

//     Ok(PyDataFrame(out))
// }

#[pymodule]
fn wrapped_example_core(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(df_sum_scores, m)?)?;
    Ok(())
}