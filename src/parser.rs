use std::collections::HashMap;

use crate::utils::Or;


#[derive(Clone, Debug)]
pub enum ParsedExpr {
    OpeningTag(String, HashMap<String, String>),
    ClosingTag(String, HashMap<String, String>),
    Expr(String),
}

pub fn parse(str: String) -> Vec<ParsedExpr> {
    let mut parsed: Vec<ParsedExpr> = Vec::from([ParsedExpr::OpeningTag("master".to_owned(), HashMap::new())]);
    let mut buffer: String = String::new();
    let mut is_closing = false;
    let mut in_tag = false;
    for c in str.chars() {
        match c {
            '<' => {
                parsed.push(ParsedExpr::Expr(buffer.clone()));
                buffer.clear();
                is_closing = false;
                in_tag = true;
            },
            '>' => {
                let (mut name, attributes) = parse_attr(buffer.clone());

                // removes / from start of closing tag
                if name.to_string().chars().collect::<Vec<char>>()[0] == '/' {
                    name.remove(0);
                }
                match is_closing {
                    true => {
                        parsed.push(ParsedExpr::ClosingTag(name, attributes));
                    },
                    false => {
                        parsed.push(ParsedExpr::OpeningTag(name, attributes));
                    }
                }
                buffer.clear();
                in_tag = false;
            },
            '/' => {
                match in_tag {
                    true => is_closing = true,
                    false => {}
                }
                buffer.push(c);
            }
            _ => {
                buffer.push(c);
            }
        }
    }
    parsed.push(ParsedExpr::ClosingTag("master".to_owned(), HashMap::new()));
    parsed
}
pub fn parse_attr(str: String) -> (String, HashMap<String, String>) {
    let name;
    let str: String = if let Some((n, out)) = str.split_once(' ') {
        name = n.to_owned();
        out.to_owned()
    } else {
        return (str, HashMap::new());
    };

    let mut attributes: HashMap<String, String> = HashMap::new();
    let mut buffer = String::new();
    let mut in_paragraph = false;
    let mut attr_buf = (String::new(), String::new());
    for c in str.chars() {
        match c {
            '=' => {
                attr_buf.0 = buffer.clone();
                buffer.clear();
            },
            '"' => {
                if in_paragraph {
                    attr_buf.1 = buffer.clone();
                    attributes.insert(attr_buf.clone().0, attr_buf.clone().1);
                    buffer.clear();
                }
                in_paragraph = !in_paragraph;
            },
            _ => { buffer.push(c) }
        }
    }
    (name, attributes)
}
#[derive(Debug, Clone)]
pub struct ParsedExprNode {
    pub name: String,
    pub attr: HashMap<String, String>,
    pub children: Vec<Or<ParsedExprNode, String>>,
}
pub fn treeify(input: Vec<ParsedExpr>) -> Vec<ParsedExprNode>{

    let mut stack: Vec<ParsedExprNode> = Vec::new();
    for expr in input {
        match expr {
            ParsedExpr::Expr(v) => {
                if !stack.is_empty(){
                    let l = stack.clone().len() - 1;
                    stack[l].children.push(Or::That(v));
                }
            },
            ParsedExpr::OpeningTag(name, attr) => {
                stack.push(ParsedExprNode {
                    name, attr,
                    children: Vec::new()
                });
            },
            ParsedExpr::ClosingTag(name, attr) => {

            },
        }
    }
    stack
}
