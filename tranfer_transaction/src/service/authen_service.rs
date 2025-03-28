use std::{env, sync::Arc};

use anyhow::{Error, Result};
use chrono::{Duration, Utc};
use tracing::info;

use crate::{
    backend::{
        argon2_hashing::verify,
        jwt_authentication::{
            claim::Claim, generate_token, passport_model::Passport, verify_token,
        },
    },
    domain::{models::authen_model::AuthenModel, repo::authen_repo::IntoAuthen},
};
pub struct AuthenService<T>
where
    T: IntoAuthen + Send + Sync,
{
    user_repo: Arc<T>,
}

impl<T> AuthenService<T>
where
    T: IntoAuthen + Send + Sync,
{
    pub fn new(user_repo: Arc<T>) -> Self {
        Self { user_repo }
    }

    pub async fn login_service(&self, user_login_model: AuthenModel) -> Result<Passport> {
        let user = &user_login_model.username;
        let password = &user_login_model.password;
        let user_login_entity = self.user_repo.find_by_username(user).await?;
        // find passwrod
        match verify(
            password.to_string(),
            user_login_entity.password_hash.to_string(),
        ) {
            Ok(_) => {
                info!("autheb success");

                let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET not set in .env");
                let access_token_claims = Claim {
                    id: user_login_entity.id,
                    user: user.to_string(),
                    exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
                    iat: Utc::now().timestamp() as usize,
                };

                let refresh_token_claims = Claim {
                    id: user_login_entity.id,
                    user: user.to_string(),
                    exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
                    iat: Utc::now().timestamp() as usize,
                };

                let access_token = generate_token(secret.clone(), &access_token_claims)?;
                let refresh_token = generate_token(secret.clone(), &refresh_token_claims)?;

                Ok(Passport {
                    access_token: access_token,
                    refresh_token: refresh_token,
                })
            }
            Err(err) => {info!("autheb fail");Err(err)},
        }
    }

    pub async fn refresh_token_service(&self, passport: Passport) -> Result<Passport> {
        let secret = env::var("TOKEN_SECRET").expect("TOKEN_SECRET not set in .env");
        let refresh_token = passport.refresh_token;

        let claim = verify_token(secret.clone(), refresh_token)?;
        let access_token_claims = Claim {
            id: claim.id,
            user: claim.user.to_string(),
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claim {
            id: claim.id,
            user: claim.user.to_string(),
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token = generate_token(secret.clone(), &access_token_claims)?;
        let refresh_token = generate_token(secret.clone(), &refresh_token_claims)?;
        Ok(Passport {
            access_token: access_token,
            refresh_token: refresh_token,
        })
    }
}
