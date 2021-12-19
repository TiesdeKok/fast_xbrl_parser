pub mod load {

    use std::fs::{self};
    use std::io::{Read};
    use std::path::PathBuf;
    use reqwest::blocking::Client;
    use crate::helpers::edgar;
    use crate::helpers::settings::AppConfig;

    pub fn download(url : String, settings : AppConfig) -> String {   
        let client = Client::new();
        let mut resp = client.get(url).header("User-Agent", settings.user_agent).send().expect("Failed to send request");


        // check status code
        if resp.status().is_success() {
            let mut body = String::new();
            resp.read_to_string(&mut body).expect("Failed to read response");
            let raw_xml = body;
            return raw_xml;
        } else {
            println!("{}", resp.status());
            panic!("Failed to download XML file, did you set a validuser agent?");
        }
    }

    pub fn load_json(file_path : PathBuf) -> String {

        let json_str = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

        return json_str
    }

    pub fn load_xml(raw_xml_path : String) -> String {

        let raw_xml = fs::read_to_string(raw_xml_path)
        .expect("Something went wrong reading the file");

        return raw_xml
    }

}

pub mod save {
    use crate::parser::xml::{XBRLFiling, FactItem};
    use std::fs::{self, File};
    use std::io::{Write};
    use std::path::{PathBuf};
    use crate::helpers::edgar;

    pub fn save_filing(file_path : PathBuf, contents : XBRLFiling) {
        //let file_path = url_data.file_path.expect("File Path not defined");
        let out_folder = file_path.parent().expect("No parent folder found"); 
        fs::create_dir_all(&out_folder).expect("Failed to create directory");

        let json_str = serde_json::to_string(&contents).expect("Failed to serialize facts");
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(json_str.as_bytes()).expect("Failed to write to file");
    }

    pub fn save_facts(file_path : PathBuf, contents : Vec<FactItem>) {
        //let file_path = url_data.file_path.expect("File Path not defined");
        let out_folder = file_path.parent().expect("No parent folder found"); 
        fs::create_dir_all(&out_folder).expect("Failed to create directory");

        let json_str = serde_json::to_string(&contents).expect("Failed to serialize facts");
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(json_str.as_bytes()).expect("Failed to write to file");
    }

    pub fn save_facts_only(folder : PathBuf, filename : String, contents : Vec<FactItem>) {

    }

}