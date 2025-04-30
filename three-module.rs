use reqwest::Error;

async fn get_top_offers() -> Result<String, Error> {
    let url = "https://plus.three.ie/core/offers/top";

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let body = response.text().await?;
    Ok(body)
}


#[pyfunction]
fn fetch_top_offers() -> PyResult<String> {
    // Create a tokio runtime
    let rt = Runtime::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let result = rt.block_on(get_top_offers());
    match result {
        Ok(body) => Ok(body),
        Err(err) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err.to_string())),
    }
}

#[pymodule]
fn top_offers(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fetch_top_offers, m)?)?;
    Ok(())
}
