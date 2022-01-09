
use fast_xbrl_parser::parser::xml::XBRLFiling;

use pyo3::prelude::*;
use pythonize::{pythonize};
use pyo3::types::IntoPyDict;

// mod helpers;
// mod parser;
// mod io;

#[pyfunction(email = "\"no_email\"", output = "[\"json\"].to_vec()")]
fn parse(input : String, email : &str, output  : Vec<&str>) -> PyResult<Py<PyAny>> {
    // Initiate Python
    let gil = Python::acquire_gil();
    let py = gil.python();

    // Parse input

    let filing = XBRLFiling::new(input.clone(), email.to_string(), output);

    // Return to Python 

    let mut python_obj = pythonize(py, &filing).unwrap();

    // This will remove any keys with empty values from the return dictionary 
    let locals = [("filing", python_obj)].into_py_dict(py);
    let result = py.eval("{k:v for k,v in filing.items() if v}", None, Some(locals)).expect("Failed to evaluate Python");

    python_obj = result.extract().unwrap();

    Ok(python_obj)

    // TODO #1: Dimensions doesn't work?
    // TODO #2: How to deal with missing values?

}

// Set up the module 

#[pymodule]
fn fast_xbrl_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}