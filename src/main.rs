use tungstenite::{connect, Message, WebSocket};
use tungstenite::client::AutoStream;
use url::Url;

use crate::models::{LimitOrderBook, OrderData, LimitOrder};
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

fn populate_limit_order(order_state_raw: &mut serde_json::Value) -> LimitOrder {
	let data = order_state_raw.get("params").expect("Could not get params from JSON object.")
								.get("data").expect("Could not get data from JSON object.");
	
	let mut limit_order = LimitOrder {
		order_type: data.get("type").expect("Could not get order type").as_str().unwrap().to_string(),
    	timestamp: data.get("timestamp").expect("Could not get timestamp").as_u64().unwrap(),
    	prev_change_id: data.get("prev_change_id").expect("Could not get previous change id").as_u64().unwrap(),
    	change_id: data.get("change_id").expect("Could not get change id").as_u64().unwrap(),
    	bids: Vec::new(),
    	asks: Vec::new(),
	};

	let asks = data.get("asks").expect("Error extracting asks").as_array().expect("Error extracting array from Value");
	let bids = data.get("bids").expect("Error extracting bids").as_array().expect("Error extracting array from Value");

	push_to_limit_order(asks, &mut limit_order, false);
	push_to_limit_order(bids, &mut limit_order, true);

	limit_order
}

fn push_to_limit_order(data: &Vec<serde_json::Value>, limit_order: &mut LimitOrder, is_bid: bool) {
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

fn update_order_book(limit_order_book: &mut LimitOrderBook, limit_order: &LimitOrder) {
	update_orders(&mut limit_order_book.bids, &limit_order.bids, true);
	update_orders(&mut limit_order_book.asks, &limit_order.asks, false);
}

fn handle_new_order(limit_order_book_orders: &mut Vec<OrderData>, order: &OrderData, is_bid: bool) {
	let new_limit_order = OrderData {
		order_type: order.order_type.clone(),
		price: order.price,
		size: order.size,
	};
	limit_order_book_orders.push(new_limit_order);
	if is_bid {
		limit_order_book_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap() );
	} else {
		limit_order_book_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap() );
	}
}

fn handle_change_order(limit_order_book_orders: &mut Vec<OrderData>, order: &OrderData) {
	let mut i = 0;
	while i < limit_order_book_orders.len() {
		if limit_order_book_orders[i].price == order.price {
			limit_order_book_orders[i].size = order.size;
		}
		i += 1;
	}
}

fn handle_delete_order(limit_order_book_orders: &mut Vec<OrderData>, order: &OrderData) {
	let index = limit_order_book_orders.iter().position(|book_orders| book_orders.price == order.price).unwrap();
	limit_order_book_orders.remove(index);
}

fn update_orders(limit_order_book_orders: &mut Vec<OrderData>, limit_orders: &Vec<OrderData>, is_bid: bool) {
	for order in limit_orders {
		match order.order_type.as_str() {
			"new" => handle_new_order(limit_order_book_orders, &order, is_bid),
			"change" => handle_change_order(limit_order_book_orders, &order),
			"delete" => handle_delete_order(limit_order_book_orders, &order),
			_ => {}
		}
		
	}
}

fn get_limit_order_book(socket: &mut WebSocket<AutoStream>) -> LimitOrderBook {
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

	let asks = extract_data(&order_book_state_raw, "asks");
	let bids = extract_data(&order_book_state_raw, "bids");

	let mut limit_order_book = LimitOrderBook { bids: Vec::new(), asks: Vec::new() };
	push_data_to_order_book(&mut limit_order_book, bids, true);
	push_data_to_order_book(&mut limit_order_book, asks, false);

	limit_order_book
}

fn main() {
    let deribit_url = format!(
        "{}/public/subscribe",
        DERIBIT_WS_API
    );
    let (mut socket, response) = connect(Url::parse(&deribit_url).unwrap()).expect("Can't connect to deribit websocket.");

    println!("Connected to Deribit stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

	let mut limit_order_book = get_limit_order_book(&mut socket);
	let mut limit_orders: Vec<LimitOrder> = Vec::new();
    loop {
		let msg = get_msg(&mut socket);
		let mut order_state_raw: serde_json::Value = serde_json::from_str(msg.as_str()).expect("Error parsing JSON");
		let limit_order: LimitOrder = populate_limit_order(&mut order_state_raw);

		if limit_orders.len() > 1 && limit_order.prev_change_id != limit_orders.get(limit_orders.len() - 1).unwrap().change_id {
			println!("Error: Packet lost reconnecting: prev change id = {}, cur change id = {}", limit_order.prev_change_id, limit_orders.get(limit_orders.len() - 1).unwrap().change_id);
			let (mut socket, _response) = connect(Url::parse(&deribit_url).unwrap()).expect("Can't connect to deribit websocket.");
			let mut limit_order_book = get_limit_order_book(&mut socket);
		}
		
		update_order_book(&mut limit_order_book, &limit_order);
		println!("Best Bid: price = {:#?} quantity = {:#?}; Best Ask: price = {:#?} quantity = {:#?}", limit_order_book.bids.get(0).unwrap().price, limit_order_book.bids.get(0).unwrap().size, limit_order_book.asks.get(0).unwrap().price, limit_order_book.asks.get(0).unwrap().size);
		limit_orders.push(limit_order);
    }
}