pub mod edgar {
    use serde::{Serialize};
    use std::path::{PathBuf};
    use std::fs::{self};
    use pyo3::prelude::*;

    #[pyclass]
    #[derive(Clone, Debug, Serialize)]
    pub struct EdgarUrl {  
        #[pyo3(get)]
        pub raw_url : String,
        #[pyo3(get)]
        pub cik : u64,
        #[pyo3(get)]
        pub cik_padded : String,
        #[pyo3(get)]
        pub accession_number : String,
        #[pyo3(get)]
        pub unique_id : String,
        #[pyo3(get)]
        pub done : bool,
        #[pyo3(get)]
        pub file_path : PathBuf
    }


    impl EdgarUrl {
        pub fn new(raw_url : String, settings : super::settings::AppConfig) -> EdgarUrl {
            let raw_cik = raw_url.split("/").collect::<Vec<&str>>()[6].clone();
            let cik = raw_cik.parse().expect("Can't parse CIK");
            let cik_padded = format!("{:0>10}", &cik);
            let accession_number = raw_url.split("/").collect::<Vec<&str>>()[7].clone();
            let unique_id = format!("{}-=-{}", cik, accession_number);

            let file_path = settings.store_location
            .join(&cik_padded).join(format!("{}.json", unique_id));
            let exists = fs::metadata(&file_path).is_ok();

            EdgarUrl {
                raw_url : raw_url.clone(),
                cik : cik,
                cik_padded : cik_padded,
                accession_number : accession_number.to_string(),
                unique_id : unique_id,
                done: exists,   
                file_path : file_path
            }
        }

        pub fn _pretty_print(&self) {
            // print all the information in the struct in a pretty format with human readable labels
            println!("CIK: {}", self.cik);
            println!("CIK Padded: {}", self.cik_padded);
            println!("Accession Number: {}", self.accession_number);
            println!("Raw URL: {}", self.raw_url);
            println!("Unique ID: {}", self.unique_id);
        }
    }

    pub fn parse_url(url : String, settings: super::settings::AppConfig) -> EdgarUrl {        
        EdgarUrl::new(url, settings)
    }

}

pub mod settings {
    use std::path::{PathBuf};

    #[derive(Clone, Debug)]
    pub struct AppConfig {
        pub user_agent: String,
        pub store_location: PathBuf,
        pub verbose: u8
    }

    impl AppConfig {
        pub fn default() ->  AppConfig{
            let mut config_settings = config::Config::default();
            config_settings.merge(config::File::with_name("Settings")).unwrap();  

            AppConfig {
                user_agent : config_settings.get_str("user_agent").expect("No user agent in config..."),
                store_location: PathBuf::from(config_settings.get_str("store_location").expect("No user agent in config...")),
                verbose : 0
            }
        }
    }
}