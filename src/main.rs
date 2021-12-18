use clap::Parser;
use std::path::{PathBuf};

mod helpers;
mod parser;
mod io;
use crate::helpers::{edgar, settings};
use crate::parser::xml::XBRLFiling;

const _VERBOSE : u8 = 0;

#[derive(Parser, Debug)]
#[clap(about, author)]
struct Args {
    /// URL to retrieve
    #[clap(short, long, default_value = "https://www.sec.gov/Archives/edgar/data/1765651/000164033421003161/pse_10k_htm.xml")]
    url: String,

    /// User agent
    #[clap(long, default_value = "config_value")]
    user_agent: String,

    /// Saving location
    #[clap(short, long, default_value = "config_value")]
    store_location: String
}

fn main() {
    // -- Deal with arguments and config file -- 

    // Initiaize config
    let mut app_settings = settings::AppConfig::default();

    // Get the arguments
    let args = Args::parse();

    if args.user_agent != "config_value".to_string() {
        app_settings.user_agent = args.user_agent.clone();
    }

    if args.store_location != "config_value".to_string() {
        app_settings.store_location = PathBuf::from(args.store_location.clone());
    }

    // Process the URL

    let url_data = edgar::parse_url(args.url.to_string(), app_settings.clone());

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
        let raw_xml = io::load::download(args.url, app_settings.clone());
        filing = XBRLFiling::parse(raw_xml, url_data.clone());
    }

    filing.pretty_print();

    // Save to JSON file

    io::save::save(url_data.clone(), filing, app_settings.clone());
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn download_url() {
        let url = "https://www.sec.gov/Archives/edgar/data/51143/000155837021004922/ibm-20210331x10q_htm.xml";
        let settings = super::settings::AppConfig::default();
        let url_data = super::edgar::parse_url(url.to_string(), settings.clone());

        assert_eq!(url_data.raw_url, url.to_string());
        }
}





// Links:

// https://www.sec.gov/Archives/edgar/data/51143/000155837021004922/ibm-20210331x10q_htm.xml

//https://www.sec.gov/Archives/edgar/data/0000051143/000155837021004922/0001558370-21-004922-index.htm