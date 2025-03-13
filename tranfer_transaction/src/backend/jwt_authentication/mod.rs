use anyhow::Result;
use claim::Claim;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

pub mod claim;
pub mod passport_model;



pub fn generate_token(secret: String, claims: &Claim) -> Result<String> {
    // HSA256
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_token(secret: String, token: String) -> Result<Claim> {
    let token = decode::<Claim>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}