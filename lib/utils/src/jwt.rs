use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Serialize, Deserialize};

const SECRET: &[u8] = b"supersecretkey";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub user_id: i32,
    pub role: String,
    pub exp: usize,
}

pub fn create_token(
    user_id: i32, 
    role: String,
) -> String {
    let claims = Claims {
        user_id,
        role,
        exp: 2000000000,
    };

    encode(
    &Header::default(),                         
        &claims,
        &EncodingKey::from_secret(SECRET),
    ).unwrap()
}

pub fn decode_token(
    token: &str,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),  
        &Validation::default(),
    )
}

pub fn verify_token(
    token: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;

    Ok(decoded.claims)
}