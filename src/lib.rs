pub mod _monte_carlo_permutation;

use pyo3::prelude::*;
use crate::_monte_carlo_permutation::mcp;


#[pymodule]
fn monte_carlo_permutation(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mcp, m)?)?;
    Ok(())
}
