//mod crate::helpers;

pub mod xml {

    const _VERBOSE : u8 = 0;

    // Imports
    use std::collections::HashMap;
    use regex::Regex;
    use serde::{Serialize, Deserialize};
    use crate::helpers::input::{Input, InputDetails, InputType};
    use crate::io;

    // Define structs

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Dimension {
        pub key_ns : String,
        pub key_value : String,
        pub member_ns : String,
        pub member_value : String
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Unit {
        pub unit_type : String,
        pub unit_value : String
    }

    impl Unit {
        pub fn to_string(&self) -> String {
            let mut unit_string = String::new();
            unit_string.push_str(&self.unit_type);
            unit_string.push_str(" -- ");
            unit_string.push_str(&self.unit_value);
            unit_string
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Period {
        pub period_type : String,
        pub period_value : String
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct FactItem {
        pub id : String,
        pub prefix: String,
        pub name : String,
        pub value : String,
        pub decimals : String,
        pub context_ref : Option<String>,
        pub unit_ref : Option<String>,
        pub dimensions : Vec<Dimension>,
        pub units : Vec<Unit>,
        pub periods : Vec<Period>
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

    // Logic for dimensions table

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct DimensionTableRow {
        pub cik : Option<String>,
        pub accession_number : Option<String>,
        pub xml_name : String,
        pub fact_id : String,
        pub left_tag : String,
        pub left_prefix : String,
        pub right_tag : String,
        pub right_prefix : String,
    }

    impl DimensionTableRow {
        pub fn default() -> DimensionTableRow {
            DimensionTableRow {
                cik : None,
                accession_number : None,
                xml_name : "".to_string(),
                fact_id : "".to_string(),
                left_tag : "".to_string(),
                left_prefix : "".to_string(),
                right_tag : "".to_string(),
                right_prefix : "".to_string()
            }
        }
    }

    pub fn dimensions_to_table(facts : Vec<FactItem>, input_details : InputDetails) ->  Vec<DimensionTableRow>{
        let mut table_rows : Vec<DimensionTableRow> = Vec::new();

        // Add rows
        for fact in facts {
            
            // THIS IS NOT YET IMPLEMENTED, JUST A PLACEHOLDER

            let mut row = DimensionTableRow::default();
            row.cik = input_details.cik.clone();
            row.accession_number = input_details.accession_number.clone();
            row.xml_name = input_details.xml_name.clone();
            row.fact_id = fact.id.clone();

            table_rows.push(row);
        }


        table_rows
    }

    // Logic for facts table

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FactTableRow {
        pub cik : Option<String>,
        pub accession_number : Option<String>,
        pub xml_name : String,
        pub fact_id : String,
        pub tag : String,
        pub value : String,
        pub prefix : String,
        pub prefix_type : String,
        pub period_start : Option<String>,
        pub period_end : Option<String>,
        pub point_in_time : Option<String>,
        pub unit : Option<String>,
        pub num_dim : u32,
    }

    impl FactTableRow {
        fn default() -> FactTableRow {
            FactTableRow {
                cik : None,
                accession_number : None,
                xml_name : "".to_string(),
                fact_id : "".to_string(),
                tag : "".to_string(),
                value : "".to_string(),
                prefix : "".to_string(),
                prefix_type : "".to_string(),
                period_start : None,
                period_end : None,
                point_in_time : None,
                unit : None,
                num_dim : 0
            }
        }
    }

    pub fn facts_to_table(facts : Vec<FactItem>, input_details : InputDetails) ->  Vec<FactTableRow>{
        let mut table_rows : Vec<FactTableRow> = Vec::new();

        let standard_tags = ["us-gaap", "dei"];

        // Add rows
        for fact in facts {
            let prefix_type = if standard_tags.contains(&fact.prefix.as_str()) {"standard"} else {"custom"};

            let mut row = FactTableRow::default();
            row.cik = input_details.cik.clone();
            row.accession_number = input_details.accession_number.clone();
            row.xml_name = input_details.xml_name.clone();
            row.fact_id = fact.id.clone();
            row.tag = fact.name.clone();
            row.value = fact.value.clone();
            row.prefix = fact.prefix.clone();
            row.prefix_type = prefix_type.to_string();
            row.num_dim = fact.dimensions.len() as u32;

            // Periods are processed into three different columns
            for period in &fact.periods {
                match period.period_type.as_str() {
                    "startDate" => row.period_start = Some(period.period_value.clone()),
                    "endDate" => row.period_end = Some(period.period_value.clone()),
                    "instant" => row.point_in_time = Some(period.period_value.clone()),
                    _ => {}
                }
            }; 
        
            // The units are converted into a single string
            if fact.units.len() > 0 {
                let tmp = fact.units.clone()
                .into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" || ");

                row.unit = Some(tmp.clone());
            }

            table_rows.push(row);
        }


        table_rows
    }

    pub fn parse_xml_to_facts(raw_xml : String) -> Vec<FactItem> {

        // -- Parse the XML --
        let re = Regex::new(r"\s+").unwrap();
        let raw_xml = re.replace_all(raw_xml.as_str(), " ").to_string();
    
        let xml_tree = roxmltree::Document::parse(raw_xml.as_str()).expect("Error parsing XML"); // Error handling?

        // -- Get elements out of XML --

        let elem = xml_tree.root_element().children().filter(|e| e.node_type() == roxmltree::NodeType::Element);

        // -- Process the context elements --
    
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
    
        // -- Process the fact elements --

        let mut facts: Vec<FactItem> = Vec::new();
    
        let non_fact_ele = ["context", "unit", "xbrl", "schemaRef"];
        let fact_ele = elem.clone().filter(|e| !&non_fact_ele.contains(&e.tag_name().name()) && e.tag_name().namespace().is_some());
    
        // loop over fact_ele using enumerate
        '_fact_loop: for (_i, child) in fact_ele.enumerate() {
            let id = child.attribute("id").unwrap_or(""); // Issue here
            let name: String = child.tag_name().name().to_string();
            let namespace: String = child.tag_name().namespace().unwrap_or("").to_string();
            let prefix = child.lookup_prefix(namespace.as_str()).unwrap_or(""); 
            let context_ref = &child.attribute("contextRef");
            let unit_ref = &child.attribute("unitRef");
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
    
            // Push to vector
    
            facts.push(FactItem {
                id : id.to_string(),
                prefix: prefix.to_string(),
                name : name.to_string(),
                value : value.to_string(),
                decimals : decimals.to_string(),
                context_ref : context_ref.map(str::to_string),
                unit_ref : unit_ref.map(str::to_string),
                units : fact_units,
                dimensions : fact_dimensions,
                periods : fact_periods,
                ..FactItem::default()
            });
        } 

        return facts;
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct XBRLFiling {
        pub info : InputDetails,
        pub json : Option<Vec<FactItem>>,
        pub facts : Option<Vec<FactTableRow>>,
        pub dimensions : Option<Vec<DimensionTableRow>>
    }

    impl XBRLFiling {
        pub fn new(input: String, email: String, output: Vec<&str>) -> XBRLFiling {

            // Parse input

            let input_info = Input::new(input.clone());

            // Get XML data
            
            let raw_xml : String = match input_info.input_type {
                InputType::Remote => {
                    io::load::download(input.clone(), email.to_string())
                }
                InputType::Local => {
                    io::load::load_xml(input.clone())
                }
            };

            // Init filing object

            let mut filing_obj = XBRLFiling {
                info : input_info.clone(),
                json : None,
                facts : None,
                dimensions : None
            };

            // Parse the XML and generate JSON
            let output_options = ["json", "facts", "dimensions"];

            // If output contains any of output_options
            if output.iter().any(|x| output_options.contains(x)) {
                let json = parse_xml_to_facts(raw_xml);

                // Return json if in output
                if output.contains(&"json") {
                    filing_obj.json = Some(json.clone());
                }

                // Parse facts tables
    
                if output.contains(&"facts") {
                    let facts_table = facts_to_table(json.clone(), input_info.clone());
                    filing_obj.facts = Some(facts_table);
                }
    
                // Parse dimensions tables
    
                if output.contains(&"dimensions") {
                    let dimensions_table = dimensions_to_table(json.clone(), input_info.clone());
                    filing_obj.dimensions = Some(dimensions_table);
                }

            }

            // Return 

            filing_obj

        }

        pub fn _pretty_print(&self) {
            // print all the information in the struct in a pretty format with human readable labels
            self.info._pretty_print();
            //println!("Number of facts found: {}", self.num_facts);
        }

    }
    
}