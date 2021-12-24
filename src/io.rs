pub mod load {

    use std::fs::{self};
    use std::io::{Read};
    use reqwest::blocking::Client;
    use crate::helpers;

    pub fn download(url : String, email : String) -> String { 
        
        let ua = helpers::input::gen_ua(email);

        let client = Client::new();
        let mut resp = client.get(url).header("User-Agent", ua).send().expect("Failed to send request");

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

    pub fn load_xml(raw_xml_path : String) -> String {

        // TODO NOTE: Should this be changed to PathBuf for OS compatibility?

        let raw_xml = fs::read_to_string(raw_xml_path)
        .expect("Something went wrong reading the file");

        return raw_xml
    }

}

pub mod save {
    use crate::parser::xml::{FactItem, FactTableRow, DimensionTableRow};
    use std::fs::{self, File};
    use std::io::{Write};
    use std::path::{PathBuf};

    #[allow(dead_code)]
    pub enum Output {
        Json(Vec<FactItem>),
        Facts(Vec<FactTableRow>),
        Dimensions(Vec<DimensionTableRow>),
    }

    impl Output {
        #[allow(dead_code)]
        pub fn save (&self, save_dir : String, file_name : String) {
            fs::create_dir_all(&save_dir).expect("Failed to create directory");
            let file_path = PathBuf::from(save_dir).join(file_name);

            match self {
                Output::Json(v) => {
                    // Convert to JSON
                    let json_str = serde_json::to_string(&v).expect("Failed to serialize to JSON");

                    // Save to file
                    let mut file = File::create(file_path).expect("Failed to create file");
                    file.write_all(&json_str.as_bytes()).expect("Failed to write to file");
                },
                Output::Facts(v) => {
                    // Convert to CSV and write to file

                    let mut wtr = csv::WriterBuilder::new()
                    .delimiter(b',')
                    .from_path(file_path).expect("Failed to create file");
    
                    for row in v {
                        wtr.serialize(&row).expect("Failed to write to CSV");
                    }

                    wtr.flush().expect("Failed to flush CSV");
                },
                Output::Dimensions(v) => {
                    // Convert to CSV and write to file
                    let mut wtr = csv::Writer::from_path(file_path).expect("Failed to create CSV writer");

                    for row in v {
                        wtr.serialize(&row).expect("Failed to write to CSV");
                    }

                    wtr.flush().expect("Failed to flush CSV");
                }
            }
        }
    }
}