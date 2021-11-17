use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
	pub order_type: String,
    pub price: f64,
    pub size: f64,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrder {
	pub order_type: String,
    pub timestamp: u64,
    pub prev_change_id: u64,
    pub change_id: u64,
    pub bids: Vec<OrderData>,
    pub asks: Vec<OrderData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderBook {
    pub bids: Vec<OrderData>,
    pub asks: Vec<OrderData>,
}
