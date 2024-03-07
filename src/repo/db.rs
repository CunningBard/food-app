use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::{Item, Order, Store, User};

// Too lazy to implement a real database, so we'll use a HashMap instead.
pub struct DB {
    pub users: Mutex<HashMap<u32, User>>,
    pub stores: Mutex<HashMap<u32, Store>>,
    pub items: Mutex<HashMap<u32, Item>>,
    pub orders: Mutex<HashMap<u32, Order>>,
}