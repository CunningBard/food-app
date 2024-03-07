
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct UserID(pub u32);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: UserID,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub id: UserID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUser {
    pub id: UserID,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserResponse {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}
