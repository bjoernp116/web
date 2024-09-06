use std::error::Error;

mod tag;
mod parser;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {

    let resp = reqwest::blocking::get("https://example.com")?.text()?;

    let parsed = parser::parse(resp);
    let tree = parser::treeify(parsed);
    for node in tree {
        println!("{:?}", node);
    }
    Ok(())
}

