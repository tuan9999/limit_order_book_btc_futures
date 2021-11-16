use tungstenite::{connect, Message, WebSocket};
use tungstenite::client::AutoStream;
use url::Url;

use crate::models::{LimitOrderBook, OrderData};
mod models;

static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";

fn get_msg(socket: &mut WebSocket<AutoStream>) -> String {
	let msg = (*socket).read_message().expect("Error reading message");
	let msg = match msg {
		tungstenite::Message::Text(s) => s,
		_ => {
			panic!("Error getting text");
		}
	};
	msg
}

fn extract_data<'a>(limit_order_book: &'a serde_json::Value, data_type: &str) -> &'a serde_json::Value {
	limit_order_book.get("params").expect("Could not get params from JSON object.")
					.get("data").expect("Could not get data from JSON object.")
					.get(data_type).expect("Could not get bids from JSON object.")
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

fn main() {
    let deribit_url = format!(
        "{}/public/subscribe",
        DERIBIT_WS_API
    );
    let (mut socket, response) = connect(Url::parse(&deribit_url).unwrap()).expect("Can't connect.");

    println!("Connected to Deribit stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

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
	let msg = get_msg(&mut socket);
	println!("{}", msg);
	let msg = get_msg(&mut socket);
	
	let order_book_state_raw: serde_json::Value = serde_json::from_str(msg.as_str()).expect("Error parsing JSON");
	println!("Book raw = {:#?}", order_book_state_raw);
	let asks = extract_data(&order_book_state_raw, "asks");
	let bids = extract_data(&order_book_state_raw, "bids");

	let mut limit_order_book = LimitOrderBook { bids: Vec::new(), asks: Vec::new() };
	push_data_to_order_book(&mut limit_order_book, bids, true);
	push_data_to_order_book(&mut limit_order_book, asks, false);
	// println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n\nLOB = {:#?}", limit_order_book);
    loop {
		break;
		// let msg = socket.read_message().expect("Error reading message");
        // let msg = match msg {
        //     tungstenite::Message::Text(s) => s,
        //     _ => {
        //         panic!("Error getting text");
        //     }
        // };
		// println!("{}", msg);
        // let parsed: models::DepthStreamWrapper = serde_json::from_str(&msg).expect("Can't parse");
        // for i in 0..parsed.data.asks.len() {
        //     println!(
        //         "{}: {}. ask: {}, size: {}",
        //         parsed.stream, i, parsed.data.asks[i].price, parsed.data.asks[i].size
        //     );
        // }
    }
}