use std::collections::HashMap;

use crate::json::JsValue;

use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchemaNode {
    ValidNode(ValidNode),
    InvalidNode(InvalidNode),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ValidNode {
    #[serde(rename = "null")]
    NullNode(NullNode),

    #[serde(rename = "boolean")]
    BooleanNode(BooleanNode),

    #[serde(rename = "string")]
    StringNode(StringNode),

    #[serde(rename = "number")]
    NumberNode(NumberNode),

    #[serde(rename = "integer")]
    IntegerNode(IntegerNode),

    #[serde(rename = "array")]
    ArrayNode(ArrayNode),

    #[serde(rename = "object")]
    ObjectNode(ObjectNode),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvalidNode {
    #[serde(flatten)]
    fields: HashMap<String, JsValue>,
}
