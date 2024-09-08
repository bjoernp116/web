use std::{error::Error};

mod tag;
mod parser;
mod utils;
use crate::tag::Tag;
fn main() -> Result<(), Box<dyn Error>> {

    //let resp = reqwest::blocking::get("https://example.com")?.text()?;
    let resp = std::fs::read_to_string("test.html")?;

    let parsed = parser::parse(resp);
    let tree = parser::treeify(parsed);
    let head = Tag::from(tree);
    println!("{:?}", head);
    Ok(())
}

