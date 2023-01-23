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

pub type ItemStats = Vec<ItemStat>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemStat {
    #[serde(rename = "Type")]
    pub item_stat_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Rows")]
    pub rows: HashMap<String, Row>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row {
    #[serde(rename = "InterpMode")]
    pub interp_mode: InterpMode,
    pub keys: Vec<Key>,
    #[serde(rename = "DefaultValue")]
    pub default_value: f64,
    #[serde(rename = "PreInfinityExtrap")]
    pub pre_infinity_extrap: InfinityExtrap,
    #[serde(rename = "PostInfinityExtrap")]
    pub post_infinity_extrap: InfinityExtrap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Key {
    #[serde(rename = "Time")]
    pub time: f64,
    #[serde(rename = "Value")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InterpMode {
    #[serde(rename = "ERichCurveInterpMode::RCIM_Linear")]
    ERichCurveInterpModeRcimLinear,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InfinityExtrap {
    #[serde(rename = "ERichCurveExtrapolation::RCCE_Constant")]
    ERichCurveExtrapolationRcceConstant,
}
