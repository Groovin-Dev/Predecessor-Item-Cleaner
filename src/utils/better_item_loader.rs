use crate::models::{pred_item::{PredItem}, item_stats::ItemStat, item_descriptions::ItemDescription, better_item::{BetterItem, BetterStat}};

use super::{pred_item_utils::nested_cost_calc};


pub fn better_item_loader(pred_items: Vec<PredItem>, item_stats: &ItemStat, item_descriptions: &ItemDescription) -> Result<Vec<BetterItem>, Box<dyn std::error::Error>> {
	// Create a vector of BetterItem objects.
	let mut better_items: Vec<BetterItem> = Vec::new();

	// Loop over all PredItem objects.
	for p_item in pred_items.clone() {
		// Get the name
		let name = p_item.name.clone();

		// The item stats has rows. Each row is named like this "Ashbringer.CDR"
		// The first part is the name the second type is the stat
		// The rows has keys. We only care about the first one and it will be an object of time and value.
		// Time is useless to us so we only care about the value.
		// We want to get all the rows that start with the name of the PredItem.

		// Create a vector of BetterStat objects.
		let mut stats: Vec<BetterStat> = Vec::new();

		// Filter the rows to only get the ones that start with the name of the PredItem.
		let filtered_rows = item_stats.rows.iter().filter(|(key, _)| key.starts_with(&name));

		// Loop over the filtered rows.
		for (key, value) in filtered_rows {
			// Split the key into two parts. The first part is the name and the second part is the stat.
			let key_parts: Vec<&str> = key.split('.').collect();

			// Get the stat from the second part of the key.
			let stat = key_parts[1];

			// If the stat is "Cost" we want to use pred_item_utils::nested_cost_calc to get the real cost.
			if stat == "Cost" {
				// Get the real cost from the pred_item_utils::nested_cost_calc.
				let real_cost = nested_cost_calc(p_item.clone(), item_stats.clone(), pred_items.clone()).unwrap();

				// Create a BetterStat object.
				let better_stat = BetterStat {
					name: stat.to_string(),
					value: real_cost,
				};

				// Add the BetterStat object to the vector.
				stats.push(better_stat);

				// Continue to the next iteration.
				continue;
			}

			// Get the value from the first part of the value.
			let value = value.keys.first().unwrap().value;

			// Create a BetterStat object.
			let better_stat = BetterStat {
				name: stat.to_string(),
				value: value,
			};

			// Add the BetterStat object to the vector.
			stats.push(better_stat);
		}

		// println!("Loading description for {}", name);

		// Get the description from the item_descriptions.
		// The descriptions are kind of like the item_stats.
		// It has rows and the rows are named after the name of the PredItem.
		// We then want the MenuDescription SourceString
		// Its possbile that there is no description for the item.
		// In that case use an empty string.
		let description = match item_descriptions.rows.get(&name) {
			Some(row) => row.menu_description.source_string.clone(),
			None => String::new(),
		};

		// Clean the description by removing all text between any < and >. Also, replace all & with a \n.
		
		// Remove all text between any < and > using regex
		let re = regex::Regex::new(r"<[a-zA-Z/]+> ?").unwrap();
		let description = re.replace_all(&description, "");

		// Replace all & with a \n
		let description = description.replace("&", "\n");

		// Sort the stats by name.
		stats.sort_by(|a, b| a.name.cmp(&b.name));

		// Get the required item names from the PredItem.
		// If there are no required items use an empty vector.
		// The names are something like "PredItem NameHere" we only want the "NameHere".
		// Once we have the name, we can find the id of the item by looping over all the PredItems and finding the one with the same name.
		let required_items = match &p_item.clone().properties.required_items {
			Some(items) => items.iter().map(|item| {
				let item_name = item.object_name.split(' ').last().unwrap().to_string();
				let item_id = pred_items.iter().find(|p_item| p_item.name == item_name).unwrap().properties.item_id;
				(item_name, item_id)
			}).collect(),
			None => Vec::new(),
		};

		// Get the rarity of the item.
		// A PredItems rarity looks something like this: EPredItemRarityType::Epic
		// We only want the Epic part.
		let rarity = match &p_item.clone().properties.item_rarity {
			Some(rarity) => rarity.split("::").last().unwrap().to_string(),
			None => String::new(),
		};

		// Create a BetterItem object.
		let better_item = BetterItem {
			id: p_item.clone().properties.item_id,
			// pred_item: p_item,
			name,
			stats,
			description,
			required_items,
			rarity
		};

		// Add the BetterItem object to the vector.
		better_items.push(better_item);
	}

	// Return the vector of BetterItem objects.
	Ok(better_items)
}