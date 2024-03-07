use crate::models::ItemID;
use crate::models::UserID;


pub struct StoreID(pub u32);

pub struct Store {
    pub id: StoreID,
    pub name: String,
    pub owner: UserID, // link to user
    pub address: String,
    pub items: Vec<ItemID>,
    pub phone: String,
}