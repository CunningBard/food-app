use crate::models::ItemID;

pub struct OrderID(pub u32);
pub struct Order {
    pub id: OrderID,
    pub user_id: i32,
    pub store_id: i32,
    pub items: Vec<ItemID>,
    pub total: f64,
}