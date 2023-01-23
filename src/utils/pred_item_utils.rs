use crate::models::{item_stats::ItemStat, pred_item::PredItem};

pub fn get_stat(item: PredItem, stats: ItemStat, stat_to_find: &str) -> Option<f64> {
	// Get the name
	let name = item.name.clone();

	// Try to find the row that matches the name and stat to find like "Ashbringer.CDR" in lowercase.
	let row = stats.rows.iter().find(|(key, _)| key.to_lowercase() == format!("{}.{}", name, stat_to_find).to_lowercase());

	// Return the stat if we found it. Otherwise return None.
	match row {
		Some((_, value)) => Some(value.keys.first().unwrap().value),
		None => None,
	}
}

pub fn nested_cost_calc(item: PredItem, stats: ItemStat, all_items: Vec<PredItem>) -> Option<f64> {
	// Most (if not all) pred items have a cost stat.
	// If the item ALSO has required items then the cost stat is only a part of the total cost.
	// We need to calculate the total cost of the item.
	// The total cost is the cost of the item + the cost of the required items.
	// The main reason of this function is that sometimes required items also have required items which have required items and so on.

	// Get the cost of the item.
	let mut cost = get_stat(item.clone(), stats.clone(), "Cost")?;

	// If the item has required items then we need to calculate the cost of those items.
	// The required items may be None
	if let Some(required_items) = item.clone().properties.required_items.clone() {
		// Loop over all the required items.
		for required_item in required_items {
			// Unfortunatly the Predecessor devs are lazy and some items have mismatched names so we cant just use the name.
			// Get the ObjectPath from the required item.
			let object_path = required_item.object_path.clone();
			// Split by the last / to get the name.
			let name = object_path.split('/').last().unwrap();
			// Then split by the first . to get the name.
			let name = name.split('.').next().unwrap();

			// Try to find the item in the all_items vector.
			// If its there, then we can calculate the cost of the required item.
			// Otherwise we just skip it.
			if let Some(required_item) = all_items.iter().find(|item| item.name == name) {
				// Calculate the cost of the required item.
				let required_item_cost = nested_cost_calc(required_item.clone(), stats.clone(), all_items.clone());
				// If the cost is Some then add it to the total cost.
				if let Some(required_item_cost) = required_item_cost {
					cost += required_item_cost;
				}
			}
		}
	}


	// Return the total cost.
	Some(cost)
}