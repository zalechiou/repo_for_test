use pyo3::prelude::*;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use numpy::{PyArray1};

#[pyfunction]
fn rust_heavy_computation(py: Python<'_>, threads: usize) -> PyResult<()> {
    py.allow_threads(|| {
        thread::scope(|s| {
            for _ in 0..threads {
                s.spawn(|| {
                    let mut acc = 0u64;
                    for i in 0..100_000_000 {
                        acc = acc.wrapping_add(i);
                    }
                    println!("Thread done: {}", acc);
                });
            }
        });
    });
    Ok(())
}

#[pyfunction]
fn rust_async_io_simulation(py: Python<'_>, threads: usize) -> PyResult<()> {
    py.allow_threads(|| {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut handles = vec![];
            for i in 0..threads {
                handles.push(tokio::spawn(async move {
                    println!("Task {} start", i);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    println!("Task {} done", i);
                }));
            }
            for h in handles {
                let _ = h.await;
            }
        });
    });
    Ok(())
}

#[pyfunction]
fn sum_numpy_array<'py>(py: Python<'py>, arr: &PyArray1<f64>) -> PyResult<f64> {
    let slice = unsafe { arr.as_slice()? };
    Ok(slice.iter().sum())
}

#[pymodule]
fn pyo3_multithread_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_heavy_computation, m)?)?;
    m.add_function(wrap_pyfunction!(rust_async_io_simulation, m)?)?;
    m.add_function(wrap_pyfunction!(sum_numpy_array, m)?)?;
    Ok(())
}
