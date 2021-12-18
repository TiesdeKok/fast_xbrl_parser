pub mod load {

    use std::fs::{self};
    use std::io::{Read};
    use std::path::Path;
    use reqwest::blocking::Client;
    use crate::helpers::edgar;
    use crate::helpers::settings::AppConfig;


    /*
    pub fn get (url_data : edgar::EdgarUrl, settings : AppConfig) -> String {
        
        if url.done {
            println!("File already exists, skipping...");
            let filing = load_local(url_data, settings);

        } else {
            println!("Downloading {}", url_data.raw_url);
            let raw_xml = download(url_data.raw_url, settings);
        }

        "PLACEHOLDER".to_string

    }
    */

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

    pub fn load_json(url_data : edgar::EdgarUrl, settings : AppConfig) -> String {

        let json_str = fs::read_to_string(url_data.file_path)
        .expect("Something went wrong reading the file");

        return json_str
    }

    pub fn load_xml(raw_xml_path : String, settings : AppConfig) -> String {

        let raw_xml = fs::read_to_string(raw_xml_path)
        .expect("Something went wrong reading the file");

        return raw_xml
    }

}

pub mod save {
    use crate::parser::xml::XBRLFiling;
    use std::fs::{self, File};
    use std::io::{Write};
    use crate::helpers::edgar;
    use std::path::Path;
    use crate::helpers::settings::AppConfig;


    pub fn save(url_data: edgar::EdgarUrl, contents : XBRLFiling, settings : AppConfig) {
        let out_folder = url_data.file_path.parent().expect("No parent folder found"); 
        fs::create_dir_all(&out_folder).expect("Failed to create directory");

        let json_str = serde_json::to_string(&contents).expect("Failed to serialize facts");
        let mut file = File::create(url_data.file_path).expect("Failed to create file");
        file.write_all(json_str.as_bytes()).expect("Failed to write to file");
    }

}