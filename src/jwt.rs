use crate::errors::ServiceError;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::NaiveDate;

use super::models::User;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    role: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResp {
    token: String
}

pub async fn validate_token(token: &str) -> Result<bool, ServiceError> {
    let public_key: String = env::var("PUBLIC_KEY").expect("PUBLIC_KEY not found.");

    let decoded = decode::<Claims>(
                    &token,
                    &DecodingKey::from_rsa_pem((&public_key).as_ref()).unwrap(),
                    &Validation::new(Algorithm::RS256),
                );
    match decoded {
        Ok(user_data) => Ok(true),
        Err(_) => Err(ServiceError::Forbidden)
    }
}

pub fn create_jwt(user: &User) -> AuthResp {
    let one_day = 24 * 60 * 60 * 1000;
    let my_claims = Claims {
        sub: user.id.clone(),
        role: user.email.clone(),
        exp: get_expiration_time(one_day) as usize,
    };
    let private_key: String = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found.");
    let key = EncodingKey::from_rsa_pem((&private_key).as_ref()).unwrap();
    AuthResp { token: encode(&Header::new(Algorithm::RS256), &my_claims, &key).unwrap() }
}

fn get_expiration_time(time: u128) -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis() + time
}