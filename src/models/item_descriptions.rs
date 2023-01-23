// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub type ItemDescriptions = Vec<ItemDescription>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemDescription {
    #[serde(rename = "Type")]
    pub item_description_type: String,
    #[serde(rename = "Name")]
    pub name: Name,
    #[serde(rename = "Properties")]
    pub properties: Properties,
    #[serde(rename = "Rows")]
    pub rows: HashMap<String, Row>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    #[serde(rename = "RowStruct")]
    pub row_struct: RowStruct,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RowStruct {
    #[serde(rename = "ObjectName")]
    pub object_name: String,
    #[serde(rename = "ObjectPath")]
    pub object_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    #[serde(rename = "GameDescription")]
    pub game_description: Description,
    #[serde(rename = "MenuDescription")]
    pub menu_description: Description,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Description {
    #[serde(rename = "Namespace")]
    pub namespace: Name,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "SourceString")]
    pub source_string: String,
    #[serde(rename = "LocalizedString")]
    pub localized_string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Name {
    #[serde(rename = "dt_Item_Descriptions")]
    DtItemDescriptions,
}
