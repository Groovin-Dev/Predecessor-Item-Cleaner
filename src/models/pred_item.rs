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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PredItem {
    #[serde(rename = "Type")]
    pub pred_item_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Properties")]
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    #[serde(rename = "ItemId")]
    pub item_id: Option<i64>,
    #[serde(rename = "Icon")]
    pub icon: Icon,
    #[serde(rename = "IconWithBackGround")]
    pub icon_with_back_ground: Option<Icon>,
    #[serde(rename = "ItemName")]
    pub item_name: ItemName,
    #[serde(rename = "Price")]
    pub price: Option<Price>,
    #[serde(rename = "RequiredItems")]
    pub required_items: Option<Vec<Icon>>,
    #[serde(rename = "ItemEffects")]
    pub item_effects: Option<Vec<ItemEffect>>,
    #[serde(rename = "ItemRarity")]
    pub item_rarity: Option<String>,
    #[serde(rename = "ItemAggressionType")]
    pub item_aggression_type: Option<String>,
    #[serde(rename = "HeroClass")]
    pub hero_class: Option<String>,
    #[serde(rename = "CooldownTags")]
    pub cooldown_tags: Option<Vec<String>>,
    #[serde(rename = "CounterTag")]
    pub counter_tag: Option<CounterTag>,
    #[serde(rename = "BlockItems")]
    pub block_items: Option<Vec<Icon>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Icon {
    #[serde(rename = "ObjectName")]
    pub object_name: String,
    #[serde(rename = "ObjectPath")]
    pub object_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CounterTag {
    #[serde(rename = "TagName")]
    pub tag_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemEffect {
    #[serde(rename = "UniqueIdentifier")]
    pub unique_identifier: CounterTag,
    #[serde(rename = "UniqueEffect")]
    pub unique_effect: Icon,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemName {
    #[serde(rename = "Namespace")]
    pub namespace: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "SourceString")]
    pub source_string: String,
    #[serde(rename = "LocalizedString")]
    pub localized_string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Price {
    #[serde(rename = "MagnitudeType")]
    pub magnitude_type: Option<String>,
    #[serde(rename = "CurveMagnitude")]
    pub curve_magnitude: Option<CurveMagnitude>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurveMagnitude {
    #[serde(rename = "CurveTable")]
    pub curve_table: Icon,
    #[serde(rename = "RowName")]
    pub row_name: String,
}
