use std::iter::FromIterator;
use indexmap::IndexMap;
use json5::from_str;
use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LadySaying {
    like: String,
    rule: IndexMap<String, String>,
}

impl LadySaying {
    pub fn new(like: &str, rule: &[(&str, &str)]) -> Self {
        Self {
            like: like.to_string(),
            rule: IndexMap::from_iter(rule.iter().map(|(k, v)| (k.to_string(), v.to_string()))),
        }
    }
    pub fn builtin() -> Vec<Self> {
        from_str(include_str!("builtin.json5")).unwrap()
    }
}