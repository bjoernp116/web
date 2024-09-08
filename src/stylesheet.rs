use std::collections::HashMap;

pub enum CssGroup {
  Id(String),
  Class(String),
  Tag(TagType),
  Unique(String)
}
pub type HexColor = (u8, u8, u8);
pub enum CssRule {
  Color(HexColor),
  BGColor(HexColor),
  TextSize(u8),
}

pub struct StyleSheet {
  rules: HashMap<CssGroup, Vec<CssRule>>
}
impl StyleSheet {
  pub fn parse(input: String) -> StyleSheet {
    let mut block: Option<CssGroup> = None;
    let mut buffer = String::new();
    let mut rules = HashMap::new();
    for c in input.chars() {
      match c {
        ' ' => continue,
        '{' => {
          block = Some(CssGroup::from(buffer));
          buffer.clear();
        },
        '}' => {
          let parsed = Self::parse_rules(buffer);
          rules.insert()
        }
      }    
    }
  }
  fn parse_rules(input: String) -> Vec<CssRule> {
    
  }
}