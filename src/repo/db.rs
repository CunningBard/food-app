use crate::models::{Item, Order, Store, User, UserID};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Too lazy to implement a real database, so we'll use a HashMap instead.

pub struct DB {
    pub users: Mutex<HashMap<u32, User>>,
    pub stores: Mutex<HashMap<u32, Store>>,
    pub items: Mutex<HashMap<u32, Item>>,
    pub orders: Mutex<HashMap<u32, Order>>,
    pub secret: Arc<String>,
}

impl DB {
    pub fn new(secret: String) -> DB {
        DB {
            users: Mutex::new(HashMap::new()),
            stores: Mutex::new(HashMap::new()),
            items: Mutex::new(HashMap::new()),
            orders: Mutex::new(HashMap::new()),
            secret: Arc::new(secret),
        }
    }

    pub fn find_user_by_id(&self, id: UserID) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.get(&id.0).cloned()
    }

    pub fn create_user(
        &self,
        username: String,
        password: String,
        first_name: String,
        last_name: String,
        phone: String,
    ) -> UserID {
        let mut users = self.users.lock().unwrap();

        let id = users.len() as u32 + 1;

        let user = User {
            id: UserID(id),
            username,
            password,
            first_name,
            last_name,
            phone,
        };

        users.insert(id, user.clone());

        user.id
    }
}
