
use pyo3::prelude::*;
use pythonize::{depythonize, pythonize};

mod helpers;
mod parser;
mod io;
use crate::helpers::edgar;
use crate::helpers::settings::AppConfig;
use crate::parser::xml::XBRLFiling;



#[pyfunction]
fn parse_url(url : String) -> PyResult<edgar::EdgarUrl> {

    let url_data = edgar::EdgarUrl::new(url, AppConfig::default());

    Ok(url_data) //.expect("Failed to create EdgarUrl")
}

#[pyfunction]

fn get_json(url : String) -> PyResult<Py<PyAny>> {
    let app_settings = AppConfig::default();

    let url_data = edgar::parse_url(url.clone(), app_settings.clone());

    // -----------------------
    // -- Get the JSON data --
    // -----------------------

    let filing: XBRLFiling;
    if url_data.done {
        println!("File already exists, loading local");
        let raw_json = io::load::load_json(url_data.clone(), app_settings.clone());
        filing = serde_json::from_str(&raw_json).unwrap();
    } else {
        println!("File does not exists, so downloading {}", &url_data.raw_url);
        let raw_xml = io::load::download(url, app_settings.clone());
        filing = XBRLFiling::parse(raw_xml, url_data.clone());
    }   

    // Store

    io::save::save(url_data.clone(), filing.clone(), app_settings.clone());

    // -------------------------
    // -- Convert to Python Dict
    // -------------------------

    let gil = Python::acquire_gil();
    let py = gil.python();

    let python_obj = pythonize(py, &filing).unwrap();

    Ok(python_obj)


    //let json_str = serde_json::to_string(&filing).expect("Failed to serialize facts");

    //Ok(json_str)
}

// Set up the module 

#[pymodule]
fn fast_xbrl_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_url, m)?)?;
    m.add_class::<edgar::EdgarUrl>()?;
    m.add_function(wrap_pyfunction!(get_json, m)?)?;
    Ok(())
}