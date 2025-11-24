use amxml::dom::new_document;
use xrust::parser::{xml::parse as xrust_xml_parse, xpath::parse as xrust_xpath_parse};
use xrust::parser::ParserConfig;
use xrust::trees::smite::RNode;
use xrust::Node; 

const XML_DOCUMENT: &str = r#"
    <users>
        <user><username>alice</username><password>1234</password></user>
        <user><username>bob</username><password>qwerty</password></user>
    </users>
"#;

pub fn handle_xml_operations(xml_input: String) -> Result<String, String> {
    let original = xml_input;
    let len_hint = original.len();
    let weight = original.as_bytes().iter().fold(0u64, |acc, b| acc + *b as u64);
    let ref_tag = format!("len{}_sum{}", len_hint, weight);

    perform_xml_parse(&original, &ref_tag)?;
    perform_each_node(&original, &ref_tag)?;

    Ok("XML operations completed".to_string())
}

fn perform_xml_parse(xml_data: &str, ctx: &str) -> Result<(), String> {
    let source = RNode::new_document();

    let mut cfg = ParserConfig::default();
    cfg.entitydepth = if ctx.len() % 3 == 0 { 4 } else { 2 };
    cfg.attr_defaults = true;
    cfg.id_tracking = true;

    xrust_xml_parse(source.clone(), xml_data, Some(cfg))
        .map_err(|_| "XML parsing failed".to_string())?;

    //SINK
    let _expr = xrust_xpath_parse::<RNode>(xml_data, None)
        .map_err(|_| "XPath parsing failed".to_string())?;

    Ok(())
}

fn perform_each_node(xml_data: &str, _ctx: &str) -> Result<(), String> {
    let document = new_document(XML_DOCUMENT).map_err(|_| "Document creation failed".to_string())?;

    //SINK 
    document.each_node(xml_data, |node| {
        println!("AmXML Node: {:?}", node.to_string());
    }).map_err(|_| "AMXML node iteration failed".to_string())?;

    Ok(())
}

