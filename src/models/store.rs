use crate::models::ItemID;
use crate::models::UserID;

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct StoreID(pub u32);

#[derive(Debug, Clone)]
pub struct Store {
    pub id: StoreID,
    pub name: String,
    pub owner: UserID, // link to user
    pub address: String,
    pub items: Vec<ItemID>,
    pub phone: String,
}
