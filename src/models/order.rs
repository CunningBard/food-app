use crate::models::ItemID;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct OrderID(pub u32);

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderID,
    pub user_id: i32,
    pub store_id: i32,
    pub items: Vec<ItemID>,
    pub total: f64,
}
