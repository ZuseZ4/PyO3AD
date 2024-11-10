#![feature(autodiff)]
use pyo3::prelude::*;
use std::autodiff::autodiff;

#[pyfunction]
fn cos(x: f32) -> PyResult<f32> {
    Ok(dsin(x, 1.0))
}
#[autodiff(dsin, Reverse, Active, ActiveOnly)]
fn sin2(x: f32) -> f32 {
    f32::sin(x)
}

#[pyfunction]
fn sin(x: f32) -> PyResult<f32> {
    Ok(f32::sin(x))
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    println!("Hi from Rust");
    Ok((a + b + 1).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustnn(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    Ok(())
}
