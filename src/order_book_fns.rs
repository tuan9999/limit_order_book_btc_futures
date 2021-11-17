pub mod order_book_functions {
	use tungstenite::{WebSocket, client::AutoStream, Message};
	use crate::{get_msg, LimitOrderBook, OrderData};

	pub fn get_limit_order_book(socket: &mut WebSocket<AutoStream>) -> LimitOrderBook {
		socket.write_message(Message::text(r#"{
			"method": "public/subscribe",
			"params": {
			  "channels": [
				"book.BTC-PERPETUAL.raw"
			  ]
			},
			"jsonrpc": "2.0",
			"id": 3
		  }"#)).expect("Error sending request");
		let msg = get_msg(socket);
		println!("{}", msg);
		let msg = get_msg(socket);
		let order_book_state_raw: serde_json::Value = serde_json::from_str(msg.as_str()).expect("Error parsing JSON");

		get_populated_limit_order_book(&order_book_state_raw)
	}

	fn get_populated_limit_order_book(order_book_state_raw: &serde_json::Value) -> LimitOrderBook {
		let asks = extract_data(&order_book_state_raw, "asks");
		let bids = extract_data(&order_book_state_raw, "bids");
	
		let mut limit_order_book = LimitOrderBook { bids: Vec::new(), asks: Vec::new() };
		push_data_to_order_book(&mut limit_order_book, bids, true);
		push_data_to_order_book(&mut limit_order_book, asks, false);

		limit_order_book
	}

	fn push_data_to_order_book(limit_order_book: &mut LimitOrderBook, data: &serde_json::Value, is_bid: bool) {
		let data = data.as_array().expect("Error extracting array from Value");
		let mut i = 0;
		while i < data.len() {
			let order_data = OrderData {
				order_type: data.get(i).unwrap().get(0).unwrap().as_str().unwrap().to_string(),
				price: data.get(i).unwrap().get(1).unwrap().as_f64().unwrap(),
				size: data.get(i).unwrap().get(2).unwrap().as_f64().unwrap()
			};
			if is_bid {
				limit_order_book.bids.push(order_data);
			} else {
				limit_order_book.asks.push(order_data);
			}
			i += 1;
		}
	}
	
	fn extract_data<'a>(order_book_state_raw: &'a serde_json::Value, data_type: &str) -> &'a serde_json::Value {
		order_book_state_raw.get("params").expect("Could not get params from JSON object.")
						.get("data").expect("Could not get data from JSON object.")
						.get(data_type).expect("Could not get bids from JSON object.")
	}
}