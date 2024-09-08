use std::iter::Product;

use crate::parser::{ParsedExpr, ParsedExprNode};
use crate::utils::Or;

#[derive(Clone, Debug)]
pub enum TagType {
    Html,
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
    Uniqe(String)
}
impl TagType {
    pub fn from(input: String) -> TagType {
        use TagType::*;
        match input.clone().as_str() {
            "html" => Html,
            "head" => Head,
            "body" => Body,
            _ => Uniqe(input)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tag {
    one_liner: bool,
    tag_type: TagType,
    fields: Vec<Tag>,
    expressions: Vec<String>,
}
impl Tag {
    pub fn from(input: ParsedExprNode) -> Tag {
        let mut fields: Vec<Tag> = Vec::new();
        let mut expressions: Vec<String> = Vec::new();
        for field in input.clone().children {
            match field {
                Or::This(c) => fields.push(Tag::from(c)),
                Or::That(s) => expressions.push(s)
            }
        }
        let tag_type = TagType::from(input.name);
        Tag {
            one_liner: false,
            fields, expressions, tag_type
        }
    }
}
