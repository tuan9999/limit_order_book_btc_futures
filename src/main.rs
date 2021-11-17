use tungstenite::connect;
use url::Url;
use std::{time, time::Duration};

mod models;
use crate::models::{LimitOrderBook, OrderData, LimitOrder};

mod msgs;
use crate::msgs::msgs::get_msg;

mod limit_order_fns;
use crate::limit_order_fns::limit_order_functions::populate_limit_order;

mod update_order_book_fns;
use crate::update_order_book_fns::update_order_book_functions::update_order_book;

mod order_book_fns;
use crate::order_book_fns::order_book_functions::get_limit_order_book;

static ONE_SECOND: Duration = time::Duration::from_millis(1000);
static DERIBIT_WS_API: &str = "wss://www.deribit.com/ws/api/v2";

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
		// thread::sleep(ONE_SECOND);
    }
}