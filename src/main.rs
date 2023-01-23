pub mod models;
pub mod utils;

use models::item_stats::*;
use models::item_descriptions::*;
use models::pred_item::*;

use utils::better_item_loader::*;

fn load_item_stats(path: &str) -> Result<ItemStats, Box<dyn std::error::Error>> {
	let file = std::fs::File::open(path)?;
	let reader = std::io::BufReader::new(file);
	let item_stats: ItemStats = serde_json::from_reader(reader)?;
	Ok(item_stats)
}

fn load_item_descriptions(path: &str) -> Result<ItemDescriptions, Box<dyn std::error::Error>> {
	let file = std::fs::File::open(path)?;
	let reader = std::io::BufReader::new(file);
	let item_descriptions: ItemDescriptions = serde_json::from_reader(reader)?;
	Ok(item_descriptions)
}

// Unlike load_item_stats and load_item_descriptions, this function takes a folder path instead of a file path.
// The folder will be an unknown depth of subfolders, and each file in the folder will be a JSON file.
// There are a lot of useless files so when looking for a file, we want these exact criteria:
// 1. The json file WILL be an array of objects with only one object in the array.
// 2. The object will have a "Type" property with a value of "PredItem".
// If those criteria are met, we want to load the file and add the object to a vector of PredItem objects.
fn load_all_items(path: &str) -> Result<Vec<PredItem>, Box<dyn std::error::Error>> {
	let mut items = Vec::new();
	for entry in std::fs::read_dir(path)? {
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			items.append(&mut load_all_items(&path.to_str().unwrap())?);
		} else if path.is_file() {
			// Since we are looping over all files, we first want to do some basic checks to see if the file is a json file.
			// If it is not a json file, we can skip it.
			let file_name = path.file_name().unwrap().to_str().unwrap();
			if file_name.ends_with(".json") {
				let file = std::fs::File::open(path)?;
				// When reading the file we have no idea if its a PredItem or not, so we need to read it as a generic object.
				let reader = std::io::BufReader::new(file);
				let json: Vec<serde_json::Value> = serde_json::from_reader(reader)?;
				// If the file is a json file, we need to check if it is a PredItem.
				// If it is a PredItem, we want to add it to the vector.
				if json.len() == 1 {
					let item = &json[0];
					if item["Type"] == "PredItem" {
						let item: PredItem = serde_json::from_value(item.clone())?;
						items.push(item);
					}
				}
			}
		}
	}
	Ok(items)
}

fn main() {
	let data_path = std::env::current_dir().unwrap().join("data");
	let items_path = data_path.join("Items");

	let item_stats = load_item_stats(&items_path.join("ct_ItemStats.json").to_str().unwrap()).unwrap();
	let item_descriptions = load_item_descriptions(&items_path.join("dt_Item_Descriptions.json").to_str().unwrap()).unwrap();
	let items = load_all_items(&items_path.to_str().unwrap()).unwrap();

	// Get the first item_stats and item_descriptions since for some reason the file is a json array with only one object.
	// Print the length of the items vector.
	let item_stats = item_stats.first().unwrap();
	let item_descriptions = item_descriptions.first().unwrap();

	println!("item_stats: {}", item_stats.rows.len());
	println!("item_descriptions: {}", item_descriptions.rows.len());
	println!("items: {}", items.len());

	let better_items = better_item_loader(items, item_stats, item_descriptions).unwrap();

	// Create the output folder if it doesn't exist.
	let output_path = data_path.join("output");

	if !output_path.exists() {
		std::fs::create_dir(&output_path).unwrap();
	}

	// Create the items.json file.
	let items_file = std::fs::File::create(output_path.join("items.json")).unwrap();

	// Write the BetterItems to the file.
	serde_json::to_writer(items_file, &better_items).unwrap();
}
