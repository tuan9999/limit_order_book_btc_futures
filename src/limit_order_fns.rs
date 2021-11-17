pub mod limit_order_functions {
	use crate::models::{OrderData, LimitOrder};

	pub fn populate_limit_order(order_state_raw: &mut serde_json::Value) -> LimitOrder {
		let data = order_state_raw.get("params").expect("Could not get params from JSON object.")
									.get("data").expect("Could not get data from JSON object.");
		
		let mut limit_order = initialise_limit_order_struct(&data);
	
		let asks = data.get("asks").expect("Error extracting asks").as_array().expect("Error extracting array from Value");
		let bids = data.get("bids").expect("Error extracting bids").as_array().expect("Error extracting array from Value");
	
		push_to_limit_order(&mut limit_order, asks, false);
		push_to_limit_order(&mut limit_order, bids, true);
	
		limit_order
	}

	fn initialise_limit_order_struct(data: &serde_json::Value) -> LimitOrder {
		LimitOrder {
			order_type: data.get("type").expect("Could not get order type").as_str().unwrap().to_string(),
			timestamp: data.get("timestamp").expect("Could not get timestamp").as_u64().unwrap(),
			prev_change_id: data.get("prev_change_id").expect("Could not get previous change id").as_u64().unwrap(),
			change_id: data.get("change_id").expect("Could not get change id").as_u64().unwrap(),
			bids: Vec::new(),
			asks: Vec::new(),
		}
	}
	
	fn push_to_limit_order(limit_order: &mut LimitOrder, data: &Vec<serde_json::Value>, is_bid: bool) {
		let mut i = 0;
		while i < data.len() {
			let order_data = OrderData {
				order_type: data.get(i).unwrap().get(0).unwrap().as_str().unwrap().to_string(),
				price: data.get(i).unwrap().get(1).unwrap().as_f64().unwrap(),
				size: data.get(i).unwrap().get(2).unwrap().as_f64().unwrap()
			};
			if is_bid {
				limit_order.bids.push(order_data);
			} else {
				limit_order.asks.push(order_data);
			}
			i += 1;
		}
	}
}