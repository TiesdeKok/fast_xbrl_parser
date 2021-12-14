use std::{fs};
use std::collections::HashMap;

//use minidom::Node;

fn main() {

    // Load the XML file as a string
    let filename = "ibm-20210331x10q_htm.xml"; // The working directory is the directory of the cargo package .... 

    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let cont_string = contents.as_str();

    let doc = roxmltree::Document::parse(cont_string).unwrap();

    // Get the root element

    let elem = doc.descendants().filter(|e| e.node_type() == roxmltree::NodeType::Element);

    // show count of elem
    //println!("{}", &elem.count());

    /* TODO

    - [ ] Process the context refs to get things like period data
        Period nodes types instant / startDate / endDate
    - [ ] Process unitRefs to get things like unit data
    - [ ] Deal with dimensions such as segments 
        - This one is more difficult because we have to get the dimension refs
    
    */

    let mut contexts: HashMap<String,String> = HashMap::new();

    
    'main_loop: for (_i, child) in elem.enumerate() {
        let name = child.tag_name().name();
        let namespace = child.tag_name().namespace().unwrap_or("");
        let prefix = child.lookup_prefix(namespace).unwrap_or(""); 
        // Improvement? --> let prefix = child.resolve_tag_name_prefix().unwrap_or("");
        let value = child.text().unwrap_or("");
        let id = child.attribute("id").unwrap_or("");

        if name == "context" {
            //println!("Names: {} {} {}", name, namespace, prefix);
            //println!("Value: {}", value);
            println!("ID {}\n", id);
            //let mut node_desc = child.children().filter(|e| e.node_type() == roxmltree::NodeType::Element);
            //dbg!(&node_desc.next());
            
            // loop over descendants of child using enumerate
            let main_ele_to_keep = ["period", "entity", "unit", "dimensions"];
            for (_i, child_ele) in child.descendants().enumerate() {
                if main_ele_to_keep.contains(&child_ele.tag_name().name()) {

                    let to_keep = ["instant", "startDate", "endDate", "segment", "measure"];
                    // Filter the descendants of child_ele for those with .name() equal to "instant" or "startDate" or "endDate"
                    let node_desc_filtered = child_ele.descendants().filter(|e| to_keep.contains(&e.tag_name().name()));
                    
                    // loop over the filtered descendants
                    for (_i, child_ele_filtered) in node_desc_filtered.enumerate() {
                        let value = child_ele_filtered.text().unwrap_or("");
                        let name = child_ele_filtered.tag_name().name();
                        let namespace = child_ele_filtered.tag_name().namespace().unwrap_or("");
                        // Print the values
                        println!("{} {} {}", name, namespace, value);
                    }
                    //dbg!(node_desc_filtered.next());
                    //break 'main_loop;
                    //dbg!(&node_desc_name.next());
                    //dbg!(&node_desc_name.next());

            

                }
                //dbg!(&child_ele);
            }
            println!("\n----\n");

            //let node = child.descendants().find(|n| n.has_tag_name("instant")).map(|n| n.text().unwrap_or(""));
            //dbg!(node);
            //break;
            //contexts.insert(id.to_string(), value.to_string());
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
    }
}

// Duration_1_1_2021_To_3_31_2021_WD_bpg0CoUKDVIjJYfcRNA

// Links:

// https://www.sec.gov/Archives/edgar/data/51143/000155837021004922/ibm-20210331x10q_htm.xml

//https://www.sec.gov/Archives/edgar/data/0000051143/000155837021004922/0001558370-21-004922-index.htm