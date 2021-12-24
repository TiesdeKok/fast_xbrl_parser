use clap::Parser;

mod helpers;
mod parser;
mod io;
use crate::parser::xml::{XBRLFiling};
use crate::io::save::{Output};

const _DEVMODE : bool = true;

#[derive(Parser, Debug)]
#[clap(about, author)]
struct Args {
    /// Input
    #[clap(short, long, default_value = "no_input")]
    input: String,

    /// JSON flag
    #[clap(long = "json")]
    return_json: bool,

    /// facts flag 
    #[clap(long = "facts")]
    return_facts: bool,

    /// dimensions flag 
    #[clap(long = "dimensions")]
    return_dimensions: bool,

    /// email input
    #[clap(short, long, default_value = "no_email")]
    email: String,

    /// Save directory
    #[clap(short, long, default_value = "no_save_dir")]
    save_dir: String,

    /// Verbose mode
    #[clap(short, long, default_value = "0")]
    verbose: u8,

    /// Silent mode
    #[clap(long)]
    silent: bool,
}

fn main() {
    // -- Deal with arguments -- 

    let mut args = Args::parse();

    let mut output: Vec<&str> = Vec::new();

    if args.return_json {
        output.push("json");
    }

    if args.return_facts {
        output.push("facts");
    }

    if args.return_dimensions {
        output.push("dimensions");
    }

    if _DEVMODE {
        //args.input = r"F:\rust_projects\fast_xbrl_parser\tests\gme-20211030_htm.xml".to_string();
        args.input = r"https://www.sec.gov/Archives/edgar/data/1326380/000132638021000129/gme-20211030_htm.xml".to_string();
        output = vec!["json", "facts", "dimensions"];
        args.email = "ties@ties.com".to_string();
        args.save_dir = r"D:\xbrl_storage".to_string();
    }

    // Parse input

    let filing = XBRLFiling::new(args.input.clone(), args.email.to_string(), output.clone());
    
    if !args.silent {
        println!("{:?}", &filing.info);
    }

    // Save to files

    if output.contains(&"json") {
        let data = Output::Json(filing.json.clone().expect("No json"));
        let file_name = format!("{}.json", filing.info.xml_name.clone()).to_string();
        data.save(args.save_dir.clone(), file_name);
    }

    if output.contains(&"facts") {
        let data = Output::Facts(filing.facts.clone().expect("No facts"));
        let file_name = format!("facts_{}.csv", filing.info.xml_name.clone()).to_string();
        data.save(args.save_dir.clone(), file_name);
    }

    if output.contains(&"dimensions") {
        let data = Output::Dimensions(filing.dimensions.clone().expect("No dimensions"));
        let file_name = format!("dimensions_{}.csv", filing.info.xml_name.clone()).to_string();
        data.save(args.save_dir.clone(), file_name);
    }
    
}

/* TESTS - TBD

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

*/