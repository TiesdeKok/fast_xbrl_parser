
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pythonize::{depythonize, pythonize};
use std::path::{PathBuf};

mod helpers;
mod parser;
mod io;
use crate::helpers::edgar;
use crate::helpers::settings::AppConfig;
use crate::parser::xml::{XBRLFiling, parse_xml_to_facts, facts_to_table};

#[pyfunction(store = "\"no_store\"")]
fn parse_url(url : String, store : &str) -> PyResult<edgar::EdgarUrl> {
    // Initiate Python
    let gil = Python::acquire_gil();
    let py = gil.python();

    // Process arguments
    let app_settings = AppConfig {
        user_agent : "na".to_string(),
        store_location: PathBuf::from(store),
        verbose : 0
    };

    let url_data = edgar::EdgarUrl::new(url, app_settings);

    Ok(url_data) //.expect("Failed to create EdgarUrl")
}

#[pyfunction(email = "\"no_email\"", store = "\"no_store\"", output = "\"json\"")]
fn parse_xbrl(input : String, email : &str, store : &str, output  : &str) -> PyResult<Py<PyAny>> {
    // Initiate Python
    let gil = Python::acquire_gil();
    let py = gil.python();

    // Process arguments
    let app_settings = AppConfig {
        user_agent : edgar::gen_ua(email.to_string()),
        store_location: PathBuf::from(store),
        verbose : 0
    };

    // Debug
    /*
    let locals = [
        ("user_agent", edgar::gen_ua(email.to_string())),
        ("store_location", PathBuf::from(store).to_string_lossy().into_owned())
    ].into_py_dict(py);

    py.run("print('Your user agent: ', user_agent, '\\n', 'Your store location: ', store_location)", 
    None, Some(locals)).unwrap();
    */
    
    let input_data = edgar::check_input(input.clone(), true);

    let mut filing : Option<XBRLFiling> = None;
    let mut facts: Vec<parser::xml::FactItem>;
    let mut url_data : Option<edgar::EdgarUrl> = None;

    if input_data.input_type == "remote".to_string() {
        url_data = Some(edgar::parse_url(input.clone(), app_settings.clone()));
        let url_data = url_data.clone().unwrap();

        if url_data.done {
            println!("File already exists, loading local");
            let tmp = url_data.clone().file_path.expect("File path not defined");
            let raw_json = io::load::load_json(tmp);
            filing = Some(serde_json::from_str(&raw_json).unwrap());
        } else {
            println!("File does not exists, so downloading {}", &url_data.raw_url);
            let raw_xml = io::load::download(input.clone(), app_settings.clone());
            filing = Some(XBRLFiling::parse(raw_xml, url_data.clone()));

            if store != "no_store" {
                let file_path = url_data.clone().file_path.expect("File Path not defined");
                io::save::save_filing(file_path, filing.clone().unwrap());
            }
        }
        
        facts = filing.clone().unwrap().facts;


    } else {
        let tmp = PathBuf::from(input.clone());
        let filename = tmp.file_name().unwrap().to_str().unwrap().replace(".xml", ".json");
        let facts_storage_path = app_settings.store_location.join("facts_only").join(filename);

        // Load facts from local storage
        if facts_storage_path.exists() {
            let raw_json = io::load::load_json(facts_storage_path);
            facts = serde_json::from_str(&raw_json).unwrap();
        } else {
            let raw_xml = io::load::load_xml(input.clone());
            facts = parse_xml_to_facts(raw_xml);

            if store != "no_store" {
                io::save::save_facts(facts_storage_path.clone(), facts.clone());
            }
        }
    }

    // -------------------------
    // -- Convert to Python Dict
    // -------------------------

    let mut filing_id : String;
    let mut cik : Option<u64> = None;
    if filing.is_some() {
        filing_id = filing.clone().unwrap().accession_number;
        cik = Some(filing.clone().unwrap().cik);
    } else {
        let tmp = PathBuf::from(input.clone());
        filing_id = tmp.file_name().unwrap().to_str().unwrap().to_string();
    }


    if output == "facts" {
        let python_obj = pythonize(py, &facts_to_table(facts.clone(), filing_id.clone(), cik.clone())).unwrap();
        let pandas = PyModule::import(py, "pandas")?;
        let df = pandas.call(
            "DataFrame",
            (python_obj,),
            None
        ).unwrap();

        Ok(df.into_py(py))

        /*
        let locals = [
            ("data", python_obj),
        ].into_py_dict(py);
        py.run("pd.DataFrame(data)", None, Some(locals)).unwrap();
        */
    } else {
        let python_obj : Py<PyAny>;
        if filing.is_some() {
            python_obj = pythonize(py, &filing).unwrap();
        } else {
            python_obj = pythonize(py, &facts).unwrap();
        }

        Ok(python_obj)

    }
    


    //let json_str = serde_json::to_string(&filing).expect("Failed to serialize facts");

    //Ok(json_str)
}

// Set up the module 

#[pymodule]
fn fast_xbrl_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_url, m)?)?;
    //m.add_class::<edgar::EdgarUrl>()?;
    m.add_function(wrap_pyfunction!(parse_xbrl, m)?)?;
    Ok(())
}