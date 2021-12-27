pub mod input {
    use serde::{Serialize, Deserialize};
    use std::fs::{self};
    use pyo3::prelude::*;

    // Input types

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub enum InputType {
        Local,
        Remote
    }

    // Logic to process the input string 

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Input {
        pub input : String,
        pub input_type : InputType,
        pub valid : bool
    }

    impl Input {
        pub fn parse_input(input : String, raise_exception : bool) -> Input {
            let input_type = if input.contains("sec.gov"){InputType::Remote} else {InputType::Local};   
            let mut valid;
            if matches!(input_type, InputType::Remote) {
                valid = (&input[..39] == "https://www.sec.gov/Archives/edgar/data".to_string()) && (input[input.len()-4..] == ".xml".to_string());
            } else {
                // Check if file path exists
                valid = fs::metadata(&input).is_ok();
            
                // Check if file is a .xml file
                if input[input.len()-4..] != ".xml".to_string() {
                    valid = false;
                }
            }
    
            if raise_exception && !valid {
                match input_type {
                    InputType::Remote => panic!("Invalid URL, please check the URL and try again."),
                    InputType::Local => panic!("File does not exists or not an XML file.")
                }
            }
    
            Input {
                input : input,
                input_type : input_type,
                valid : valid
            }
        }

        pub fn new(input : String) -> InputDetails {
            InputDetails::new(Input::parse_input(input, true))
        }

    }

    // Parse details out of the input string

    #[pyclass]
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InputDetails {  
        pub raw_input : String,
        pub xml_name : String,
        pub input_type : InputType,
        pub cik : Option<String>,
        pub accession_number : Option<String>
    }

    impl InputDetails {
        pub fn new(input : Input) -> InputDetails {
            let raw_input = input.input.clone();
            let tmp = raw_input.replace(".xml", "");
            let xml_name = tmp.split(&['.', '/', '\\'][..]).collect::<Vec<&str>>().last().expect("No last").clone();

            let mut output =  InputDetails {
                raw_input : raw_input.clone(),
                xml_name : xml_name.clone().to_string(),
                input_type : input.input_type.clone(),
                cik : None,
                accession_number : None
            };
            
            // Add CIK and accession number if remote
            if matches!(input.input_type, InputType::Remote) {
                let raw_cik = raw_input.split("/").collect::<Vec<&str>>()[6].clone();
                let cik : u64 = raw_cik.parse().expect("Can't parse CIK");
                let cik_padded = format!("{:0>10}", cik.clone());
                let accession_number = raw_input.split("/").collect::<Vec<&str>>()[7].clone();
                //let unique_id = format!("{}-=-{}", cik, accession_number);

                output.cik = Some(cik_padded);
                output.accession_number = Some(accession_number.to_string());
            } 

            output
        }

        pub fn _pretty_print(&self) {
            // print all the information in the struct in a pretty format with human readable labels
            println!("Raw input : {}", self.raw_input);
            println!("XML name : {}", self.xml_name);
            println!("Input type : {:?}", self.input_type);
            //println!("CIK: {}", self.cik.unwrap_or("No CIK found".to_string()));
            //println!("Accession Number: {}", self.accession_number.unwrap_or("No accession number found".to_string()));
        }
    }

    // Logic to generate valid user agent string from email

    pub fn gen_ua(email: String) -> String {
        if email == "no_email" {
            panic!("The SEC requires that you provide your email address. Please provide your email address through the 'email' parameter.");
        } else {
            let name = email.split('@').collect::<Vec<&str>>()[0];
            format!("{} - {}", name, email)
        }
        
    }
}

pub mod sanitize {
    use scraper::Html;
    use regex::Regex;

    pub fn html(input : String) -> String {
        let mut output = input.clone();
        
        // Remove non ascii characters and replace them with a space
        output = output.replace(|c: char| !c.is_ascii(), " ");

        // Remove HTML
        if output.contains("<") {
            // Remove HTML tags
            let fragment = Html::parse_fragment(output.as_str());
            let root = fragment.root_element();
            output = root.text().collect::<Vec<_>>().join(" ");
        }

        // Remove duplicate white spaces

        let re = Regex::new(r"\s+").unwrap();
        output = re.replace_all(output.as_str(), " ").to_string();
        
        // Return
        return output

    }
}