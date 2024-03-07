
pub struct UserID(pub u32);

pub struct User {
    pub id: UserID,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}