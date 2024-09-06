use std::iter::Product;

use crate::parser::ParsedExpr;
use crate::utils::Or;
pub enum TagType {
    Head,
    Body,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    A,
    Meta,
    Style,
    P,
}

pub struct Tag {
    one_liner: bool,
    tag_type: TagType,
    fields: Vec<Tag>,
    expressions: Vec<String>,
}
impl Tag {
    pub fn from(input: Vec<ParsedExpr>) -> Vec<Or<Tag, String>> {
        for expr in input {
        }
        todo!()
    }
}
