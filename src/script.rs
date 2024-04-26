use pyo3::prelude::*;

#[pyfunction]
pub fn quote(mut input: Vec<String>) -> Vec<String> {
    input
        .iter_mut()
        .for_each(|elem| *elem = format!("\"{}\"", elem));

    input
}

#[pyfunction]
pub fn trim(mut input: Vec<String>) -> Vec<String> {
    input
        .iter_mut()
        .for_each(|elem| *elem = elem.trim().to_string());

    input
}

#[pyfunction]
pub fn prepend(mut input: Vec<String>, value: String) -> Vec<String> {
    input
        .iter_mut()
        .for_each(|elem| *elem = format!("{value}{}", elem));

    input
}

#[pyfunction]
pub fn append(mut input: Vec<String>, value: String) -> Vec<String> {
    input
        .iter_mut()
        .for_each(|elem| *elem = format!("{}{value}", elem));

    input
}

#[pyfunction]
pub fn normalize_spaces(mut input: Vec<String>) -> Vec<String> {
    input.iter_mut().for_each(|elem| {
        *elem = elem.split_whitespace().intersperse(" ").collect();
    });

    input
}

#[pyfunction]
pub fn uniq(mut input: Vec<String>) -> Vec<String> {
    input.dedup();
    input
}

#[pyfunction]
pub fn join_or(input: Vec<String>) -> String {
    input.join(" OR ")
}

#[pyfunction]
pub fn join_and(input: Vec<String>) -> String {
    input.join(" AND ")
}

#[pyfunction]
pub fn filter_commented(input: Vec<String>) -> Vec<String> {
    input.into_iter().filter(|elem| !elem.starts_with("#")).collect()
}

#[pyfunction]
pub fn filter_empty(input: Vec<String>) -> Vec<String> {
    input.into_iter().filter(|elem| !elem.is_empty()).collect()
}

#[pymodule]
pub fn q3(q3_module: &Bound<'_, PyModule>) -> PyResult<()> {
    q3_module.add_function(wrap_pyfunction!(quote, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(join_or, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(join_and, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(uniq, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(trim, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(normalize_spaces, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(filter_commented, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(filter_empty, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(prepend, q3_module)?)?;
    q3_module.add_function(wrap_pyfunction!(append, q3_module)?)?;
    Ok(())
}
