use pyo3::prelude::*;

extern crate image;

use image::GenericImageView;

#[pyfunction]
fn grayscale(img: &str, filename: &str) -> PyResult<bool> {
    let img = image::open(img).unwrap();
    let grey = img.grayscale();
    grey.save(filename).unwrap();
    Ok(true)
}

fn blur(img: &str, filename: &str, sigma: &f32) -> PyResult<bool> {
    let img = image::open(img).unwrap();
    let grey = img.blur(*sigma);
    grey.save(filename).unwrap();
    Ok(true)
}

fn brighten(img: &str, filename: &str, value: &i32) -> PyResult<bool> {
    let img = image::open(img).unwrap();
    let grey = img.brighen(value);
    grey.save(filename).unwrap();
    Ok(true)
}



#[pymodule]
fn filters(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(grayscale, m)?)?;
    m.add_function(wrap_pyfunction!(blur, m)?)?;
    m.add_function(wrap_pyfunction!(brighten, m)?)?;
    Ok(())
}
