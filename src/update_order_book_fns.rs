pub mod update_order_book_functions {
	use crate::{LimitOrderBook, LimitOrder, OrderData};

	pub fn update_order_book(limit_order_book: &mut LimitOrderBook, limit_order: &LimitOrder) {
		update_orders(&mut limit_order_book.bids, &limit_order.bids, true);
		update_orders(&mut limit_order_book.asks, &limit_order.asks, false);
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
}