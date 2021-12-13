use std::{fs};
use std::collections::HashMap;

use minidom::Node;

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
    
    */

    let mut contexts: HashMap<String,String> = HashMap::new();

    
    for (_i, child) in elem.enumerate() {
        let name = child.tag_name().name();
        let namespace = child.tag_name().namespace().unwrap_or("");
        let prefix = child.lookup_prefix(namespace).unwrap_or(""); 
        // Improvement? --> let prefix = child.resolve_tag_name_prefix().unwrap_or("");
        let value = child.text().unwrap_or("");
        let id = child.attribute("id").unwrap_or("");

        if name == "context" {
            //println!("Names: {} {} {}", name, namespace, prefix);
            //println!("Value: {}", value);
            //println!("ID {}", );
            //let mut node_desc = child.children().filter(|e| e.node_type() == roxmltree::NodeType::Element);
            //dbg!(&node_desc.next());
            let node = child.descendants().find(|n| n.has_tag_name("instant")).map(|n| n.text().unwrap_or(""));
            dbg!(node);
            break;
            //contexts.insert(id.to_string(), value.to_string());
         } else {
            //"GrossProfit"
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
            }
        
    }

    //doc.get_node(id)
    
    

    //dbg!(elem);


    //println!("With text:\n{}", &contents.to_string()[..100]);

    /*

    // Try the minidom crate

    let root: Element = contents.parse().unwrap();

    // Get the value
    let value = &root.text();
    println!("{:?}", value);
    
    // Get attributes
    for (ii, t_item) in root.attrs().enumerate() {
        println!("{:?}", t_item);
    }

    // Get the namespace
    let ns = &root.ns();
    dbg!(&ns);
    //for (ii, t_item) in root.attrs().enumerate() {
    //    println!("Root: {} {:?}", ii, t_item);
    //}

    //let test = root.attr("xmlns");
    //println!("Root: {:?}", test);

    let iter = root.children();

    for (i, child) in iter.enumerate() {
        if i == 0 {
            println!("{:?}", &child);

            // Get the value
            let value = &child.text();
            println!("{:?}", value);
            
            // Get attributes
            for (ii, t_item) in child.attrs().enumerate() {
                println!("{:?}", t_item);
            }

            // Get the namespace
            let ns = &child.ns();
            dbg!(&ns);

            //let prefix = &child.prefix.clone();

            // Get the prefix
            //let prefix = &child.get_prefix();

            // Get the namespace
            //let prefix = Some(child.text());
            // dbg!(prefix.unwrap_or(String::from("missing...")));
            //println!("{} {} {} {}", i, child.name(), child.prefix(), child.children()[0]);
            break;
        }

        //println!("{} {}", i, child.name());
    }

    //println!("{:?}", root);
    */
}