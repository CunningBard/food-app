#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaim {
    pub user_id: u32,
    pub exp: u64,
}
