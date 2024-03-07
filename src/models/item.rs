use crate::models::StoreID;

pub struct ItemID(pub i32);

pub struct Item {
    pub id: i32,
    pub name: String,
    pub price: f64,
    pub store_id: StoreID,
}