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
    pub timestamp: u32,
    pub prev_change_id: u32,
    pub change_id: u32,
    pub instrument_name: String,
    pub bids: Vec<OrderData>,
    pub asks: Vec<OrderData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LimitOrderBook {
    pub bids: Vec<OrderData>,
    pub asks: Vec<OrderData>,
}
