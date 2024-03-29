use pyo3::prelude::*;
use pyo3::types::PyList;
use rayon::prelude::*;
use rayon;
use num_cpus;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn add_one<'a>(input_list: &'a PyList) -> PyResult<Vec<i32>> {
    let result: Vec<i32> =
        input_list.extract::<Vec<i32>>().unwrap().iter().map(
            |x| x + 1
        ).collect();
    
    Ok(result)
}

#[pyfunction]
fn self_powering<'a>(input_list: &'a PyList) -> PyResult<Vec<u32>> {
    let result: Vec<u32> =
        input_list.extract::<Vec<u32>>().unwrap().iter().map(
            |x| u32::pow(*x, *x)
        ).collect();
    Ok(result)
}

#[pyfunction]
fn add_one_inplace<'a>(input_list: &'a PyList) -> PyResult<()> {
    for (index, item) in input_list.iter().enumerate() {
        let item = item.extract::<i32>()?;
        let new_item = item + 1;
        input_list.set_item(index, new_item)?;
    }
    Ok(())
}

#[pyfunction]
fn add_one_parallel<'a>(input_list: &'a PyList) -> PyResult<Vec<i32>> {
    // NOTE: The extraction of i32 from PyList is the bottleneck.
    let mut input: Vec<i32> = input_list.extract::<Vec<i32>>().unwrap();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() * 2)
        .build()
        .unwrap();
    pool.install(|| {
        input.par_iter_mut().for_each(|x| *x += 1);
    });
    Ok(input)
}


#[pyfunction]
fn self_powering_parallel<'a>(input_list: &'a PyList) -> PyResult<Vec<u32>> {
    // NOTE: The extraction of i32 from PyList is the bottleneck.
    let mut input: Vec<u32> = input_list.extract::<Vec<u32>>().unwrap();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() * 2)
        .build()
        .unwrap();
    pool.install(|| {
        input.par_iter_mut().for_each(|x| *x = u32::pow(*x, *x));
    });
    Ok(input)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pylist(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_one, m)?)?;
    m.add_function(wrap_pyfunction!(add_one_inplace, m)?)?;
    m.add_function(wrap_pyfunction!(add_one_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(self_powering, m)?)?;
    m.add_function(wrap_pyfunction!(self_powering_parallel, m)?)?;
    Ok(())
}