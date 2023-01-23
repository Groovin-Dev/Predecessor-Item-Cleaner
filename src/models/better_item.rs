use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BetterItem {
	pub id: Option<i64>,
	// pub pred_item: PredItem,
	pub name: String,
	pub stats: Vec<BetterStat>,
	pub description: String,
	pub required_items: Vec<(String, Option<i64>)>,
	pub rarity: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BetterStat {
	pub name: String,
	pub value: f64,
}