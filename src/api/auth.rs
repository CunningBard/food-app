use crate::models::AuthClaim;
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

const EXP_TIME: i64 = 86400; // Token expiration time in seconds

pub fn generate_token(secret_key: String, user_id: u32) -> Option<String> {
    let claims = AuthClaim {
        user_id,
        exp: (Utc::now() + TimeDelta::try_seconds(EXP_TIME).unwrap()).timestamp() as u64,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .ok()?;

    Some(token)
}

pub fn authenticate_token(secret_key: String, token: String) -> Option<u32> {
    let token_data = decode::<AuthClaim>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .ok()?;

    if token_data.claims.exp < Utc::now().timestamp() as u64 {
        None
    } else {
        Some(token_data.claims.user_id)
    }
}
