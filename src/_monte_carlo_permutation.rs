use core::f64;
use rand::seq::SliceRandom;
use rand::thread_rng;
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyfunction]
pub fn mcp(signals: Vec<f64>, returns: Vec<f64>, m: f64) -> PyResult<(f64, Vec<f64>)> {
    if signals.len() != returns.len() {
        return Err(PyValueError::new_err("Argument `signals` must share the same length with argument `returns`."))
    }
    if signals.iter().any(|s| s.is_nan()) {
        return Err(PyValueError::new_err("Argument `signals` must not contains NaN."))
    }

    if returns.iter().any(|r| r.is_nan()) {
        return Err(PyValueError::new_err("Argument `returns` must not contains Nan."))
    }

    if m <= 0.0 {
        return Err(PyValueError::new_err("Argument `m` must be greater 0."))
    }

    if m.fract() != 0.0 {
        return Err(PyValueError::new_err("Argument `m` must be integer."))
    }

    let m = m as usize;

    // Compute the maximum mean return of each signal.
    let max_mean_return = (0..signals.len())
        .map(|i| signals[i] * returns[i])
        .sum::<f64>()
        / signals.len() as f64;

    // Repeatedly sample the maximum of permutated mean for 'm' times as sample distribution.
    let mut sample_distribution = Vec::<f64>::with_capacity(m);
    for _ in 0..m {
        let mut samples = returns.clone();
        samples.shuffle(&mut thread_rng());
        let permutated_mean = (0..returns.len())
            .map(|k| signals[k] * samples[k])
            .sum::<f64>()
            / returns.len() as f64;
        sample_distribution.push(permutated_mean);
    }

    let p_values = sample_distribution
        .clone()
        .into_iter()
        .filter(|s| *s >= max_mean_return)
        .sum::<f64>()
        / m as f64;

    Ok((p_values, sample_distribution))
}