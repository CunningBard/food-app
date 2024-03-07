use crate::models::StoreID;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ItemID(pub i32);

#[derive(Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub store_id: StoreID,
}
