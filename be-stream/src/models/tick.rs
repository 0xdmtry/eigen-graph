use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Tick {
    pub product_id: String,
    pub price: String,
    pub time: String,
}
