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
	let LOB: serde_json::Value = serde_json::from_str(msg.as_str()).expect("Error parsing JSON");
	// println!("{:#?}", LOB);
	let asks = LOB.get("params").expect("Could not get params from JSON object.")
					.get("data").expect("Could not get data from JSON object.")
					.get("asks").expect("Could not get asks from JSON object.");
	let bids = LOB.get("params").expect("Could not get params from JSON object.")
					.get("data").expect("Could not get data from JSON object.")
					.get("bids").expect("Could not get bids from JSON object.");
	// println!("asks = {:#?}\n\nbids = {:#?}", asks, bids);
	let mut limit_order_book = LimitOrderBook { bids: Vec::new(), asks: Vec::new() };
	println!("bids = {:#?}", bids);
	let bids = bids.as_array().expect("could");
	let mut i = 0;
	while i < bids.len() {
		let order_data = OrderData {
			order_type: bids.get(i).unwrap().get(0).unwrap().as_str().unwrap().to_string(),
			price: bids.get(i).unwrap().get(1).unwrap().as_f64().unwrap(),
			size: bids.get(i).unwrap().get(2).unwrap().as_f64().unwrap()
		};
		println!("bid = {:#?}", bids.get(i).unwrap());
		println!("type = {:#?}", order_data);
		limit_order_book.bids.push(order_data);
		i += 1;
	}
	println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@\n\nLOB = {:#?}", limit_order_book);
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