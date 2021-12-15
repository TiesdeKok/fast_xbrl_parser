use std::{fs};
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};


//use minidom::Node;

const _VERBOSE : u8 = 0;

fn main() {

    // Load the XML file as a string
    let filename = "ibm-20210331x10q_htm.xml"; // The working directory is the directory of the cargo package .... 

    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let cont_string = contents.as_str();

    // Not sure this does anything:

    let re = Regex::new(r"\s+").unwrap();
    let cont_string = re.replace_all(cont_string, " ");

    let doc = roxmltree::Document::parse(&cont_string).unwrap();

    // Get the root element


    // show count of elem
    //println!("{}", &elem.count());

    /* TODO

    - [ ] Process the context refs to get things like period data
        Period nodes types instant / startDate / endDate
    - [ ] Process unitRefs to get things like unit data
    - [ ] Deal with dimensions such as segments 
        - This one is more difficult because we have to get the dimension refs
    
    */

    let elem = doc.root_element().children().filter(|e| e.node_type() == roxmltree::NodeType::Element);

    // -- Process the context elements --

    #[derive(Clone, Debug, Deserialize, Serialize)]

    struct Dimension {
        key_ns : String,
        key_value : String,
        member_ns : String,
        member_value : String
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]

    struct Unit {
        unit_type : String,
        unit_value : String
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]

    struct Period {
        period_type : String,
        period_value : String
    }

    let mut units: HashMap<String,Vec<Unit>> = HashMap::new();
    let mut periods: HashMap<String,Vec<Period>> = HashMap::new();
    let mut dimensions: HashMap<String,Vec<Dimension>> = HashMap::new();

    // --- Process the unit elements ---
    let unit_ele = elem.clone().filter(|e| e.tag_name().name() == "unit");
    '_unit_loop: for (_i, child) in unit_ele.enumerate() {
        let id = child.attribute("id").unwrap_or("");
        let measure_nodes = child.descendants().filter(|e| e.tag_name().name() == "measure");

        for (_i, m_ele) in measure_nodes.enumerate() {
            let name = m_ele.parent().unwrap().tag_name().name();
            let value = m_ele.text().unwrap_or("");
            units.entry(id.to_string())
            .or_default()
            .push(Unit {
                unit_type : name.to_string(),
                unit_value : value.to_string()
            });

            if _VERBOSE > 1 {println!("{} {}", m_ele.parent().unwrap().tag_name().name(), m_ele.text().unwrap_or(""));}
        }
    }

    // --- Process the context elements ---
    let context_ele = elem.clone().filter(|e| e.tag_name().name() == "context");
    '_context_loop: for (_i, child) in context_ele.enumerate() {
        
        let id = child.attribute("id").unwrap_or("");
        if _VERBOSE > 1 {println!("ID {}\n", id);}

        let node_desc = child.children().filter(|e| e.node_type() == roxmltree::NodeType::Element);

        // loop over descendants and process the different types of elements
        for (_i, child_ele) in node_desc.enumerate() {
            match child_ele.tag_name().name() {
                "period" => {
                    if _VERBOSE > 1 {println!("\n -- Found period -- \n");}

                    let to_keep = ["instant", "startDate", "endDate"];
                    let node_desc_filtered = child_ele.descendants().filter(|e| to_keep.contains(&e.tag_name().name()));
                    
                    for (_i, child_ele_filtered) in node_desc_filtered.enumerate() {
                        let value = child_ele_filtered.text().unwrap_or("");
                        let name = child_ele_filtered.tag_name().name();
                        let _namespace = child_ele_filtered.tag_name().namespace().unwrap_or("");

                        periods.entry(id.to_string())
                        .or_default()
                        .push(Period {
                            period_type : name.to_string(),
                            period_value : value.to_string()
                        });

                        if _VERBOSE > 1 {println!("Period: {} {}", name, value);}
                    }
                }
                "entity" => {
                    if _VERBOSE > 1 {println!("\n -- Found entity -- \n");}

                    let to_keep = ["explicitMember"];
                    let node_desc_filtered = child_ele.descendants().filter(|e| to_keep.contains(&e.tag_name().name()));
                    
                    for (_i, child_ele_filtered) in node_desc_filtered.enumerate() {
                        let value = child_ele_filtered.text().unwrap_or("");
                        let _name = child_ele_filtered.tag_name().name();
                        let _namespace = child_ele_filtered.tag_name().namespace().unwrap_or("");
                        if child_ele_filtered.has_attribute("dimension") {
                            let dimension_raw = child_ele_filtered.attribute("dimension").unwrap();
                            let dimension_split = dimension_raw.split(":").collect::<Vec<&str>>();
                            let dimension_ns = dimension_split[0];
                            let dimension_value = dimension_split[1];

                            let value_split = value.split(":").collect::<Vec<&str>>();
                            let key_ns = value_split[0];
                            let key_value = value_split[1];

                            dimensions.entry(id.to_string())
                            .or_default()
                            .push(Dimension {
                                key_ns : dimension_ns.to_string(),
                                key_value : dimension_value.to_string(),
                                member_ns : key_ns.to_string(),
                                member_value : key_value.to_string()
                            });
    
                            if _VERBOSE > 1 {println!("Segment: {} {} {} {}", dimension_ns, dimension_value, key_ns, key_value);}
                        }


                    }
                    
                }
                _ => {}
            }
        }
    }

    // -- Loop over the fact elements --
    
    #[derive(Debug, Serialize)]
    struct FactItem {
        id : String,
        prefix: String,
        name : String,
        value : String,
        decimals : String,
        context_ref : Option<String>,
        unit_ref : Option<String>,
        dimensions : Vec<Dimension>,
        units : Vec<Unit>,
        periods : Vec<Period>
    }

    impl FactItem {
        fn default() -> FactItem {
            FactItem {
                id : "".to_string(),
                prefix: "".to_string(),
                name : "".to_string(),
                value : "".to_string(),
                decimals : "".to_string(),
                context_ref : None,
                unit_ref : None,
                dimensions : Vec::new(),
                units : Vec::new(),
                periods : Vec::new()
            }
        }
    }

    let mut facts: Vec<FactItem> = Vec::new();

    let non_fact_ele = ["context", "unit", "xbrl", "schemaRef"];
    let fact_ele = elem.clone().filter(|e| !&non_fact_ele.contains(&e.tag_name().name()) && e.tag_name().namespace().is_some());

    // loop over fact_ele using enumerate
    '_fact_loop: for (_i, child) in fact_ele.enumerate() {
        let id = child.attribute("id").unwrap_or("");
        let name: String = child.tag_name().name().to_string();
        let namespace: String = child.tag_name().namespace().unwrap_or("").to_string();
        let prefix = child.lookup_prefix(namespace.as_str()).unwrap_or(""); 
        let context_ref = child.attribute("contextRef");
        let unit_ref = child.attribute("unitRef");
        let decimals = child.attribute("decimals").unwrap_or("");
        let value = child.text().unwrap_or("");

        let mut fact_dimensions : Vec<Dimension> = Vec::new();
        let mut fact_units : Vec<Unit> = Vec::new();
        let mut fact_periods : Vec<Period>= Vec::new();

        // Look up the units 
        if unit_ref.is_some() {
            let unit_ref_value = unit_ref.unwrap().to_string();
            // if unit_ref in units 
            if units.contains_key(&unit_ref_value) {
                fact_units = units.get(&unit_ref_value).expect("Unit not found").clone();
            }
            
        }

        // Look up the dimensions
        if context_ref.is_some() {
            let context_ref_value = context_ref.unwrap().to_string();
            if dimensions.contains_key(&context_ref_value) {
                fact_dimensions = dimensions.get(&context_ref_value).expect("Dimension not found").clone();
            }
        }

        // Look up the periods
        if context_ref.is_some() {
            let context_ref_value = context_ref.unwrap().to_string();
            if periods.contains_key(&context_ref_value) {
                fact_periods = periods.get(&context_ref_value).expect("Period not found").clone();
            }
        }

        if _VERBOSE > 0 {println!("Fact: {} {} {} {} \n {} {}", prefix, name, value, decimals, 
        context_ref.unwrap_or("no context"), unit_ref.unwrap_or("no unit"));}

        facts.push(FactItem {
            id : id.to_string(),
            prefix: prefix.to_string(),
            name : name.to_string(),
            value : value.to_string(),
            decimals : decimals.to_string(),
            //context_ref : Some(context_ref.to_string()), <--- TO FIX
            //unit_ref : unit_ref.to_string(), <--- TO FIX
            units : fact_units,
            dimensions : fact_dimensions,
            periods : fact_periods,
            ..FactItem::default()
        });

    } 
    // print the length of facts in a pretty way
    println!("\n -- Found {} facts -- \n", facts.len());

    // include serde_json


    // Save facts to json file
    let facts_json = serde_json::to_string(&facts).expect("Failed to serialize facts");
    let output_dir= r"F:\rust_projects\xbrl_parser";
    let output_file = "facts.json";
    let mut file = File::create(format!("{}/{}", output_dir, output_file)).expect("Failed to create file");
    file.write_all(facts_json.as_bytes()).expect("Failed to write to file");



    //println!("\n\n -- Found {} facts -- \n", &fact_ele.count());

    

    //dbg!(&contexts);
    //println!("\n----\n");


        //let node = child.descendants().find(|n| n.has_tag_name("instant")).map(|n| n.text().unwrap_or(""));
        //dbg!(node);
        //break;
        //contexts.insert(id.to_string(), value.to_string());


        /* 

        //let _namespace = child.tag_name().namespace().unwrap_or("");
        //let _prefix = child.lookup_prefix(namespace).unwrap_or(""); 
        // Improvement? --> let prefix = child.resolve_tag_name_prefix().unwrap_or("");
        //let _value = child.text().unwrap_or("");


        //println!("Names: {} {} {}", name, namespace, prefix);
        //println!("Value: {}", value);


         } else {
            //"GrossProfit"
            /*
            const TAG: &str = "FinancialAssetsAndLiabilitiesNotMeasuredAtFairValuePolicyTextBlock";
            if name == TAG {
                println!("Names: {} {} {}", name, namespace, prefix);
                println!("Value: {}", value);
                
                println!("\nAttribute values:");
                for attr in child.attributes() {
                    // print name and value
                    println!("{} {}", attr.name(), attr.value());
                }

                //println!("{} {:?} {:?} {:?}", i, child.tag_name(), child.text(), child.attributes());
                //println!("{}", child.lookup_prefix());
                break;
            }
            */
        }
        */
}


// Duration_1_1_2021_To_3_31_2021_WD_bpg0CoUKDVIjJYfcRNA

// Links:

// https://www.sec.gov/Archives/edgar/data/51143/000155837021004922/ibm-20210331x10q_htm.xml

//https://www.sec.gov/Archives/edgar/data/0000051143/000155837021004922/0001558370-21-004922-index.htm