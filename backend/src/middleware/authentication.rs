use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{FromRequest, HttpRequest, HttpMessage};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

use crate::utils::helpers::JWTUserClaims;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl From<serde_json::Error> for ErrorResponse {
    fn from(_: serde_json::Error) -> Self {
        ErrorResponse {
            status: "error".to_string(),
            message: "An error occurred while processing the response".to_string(),
        }
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap_or_default())
    }
}

pub struct JwtMiddleware {
    pub username: String,
}

impl FromRequest for JwtMiddleware {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| {
                if header.starts_with("Bearer ") {
                    Some(header[7..].to_string())
                } else {
                    None
                }
            });

        if let Some(token) = token {
            // let data = req.app_data::<AppState>().expect("Failed to get app state");
            let secret = DecodingKey::from_secret("your-secret-key".as_ref());
            let validation = Validation::default();

            match decode::<JWTUserClaims>(&token, &secret, &validation) {
                Ok(token_data) => {
                    let username = token_data.claims.username.to_string();
                    req.extensions_mut().insert(Username(username.clone()));
                    ready(Ok(JwtMiddleware { username }))
                }
                Err(_) => {
                    let error_response = ErrorResponse {
                        status: "error".to_string(),
                        message: "Invalid token".to_string(),
                    };
                   ready(Err(ErrorUnauthorized::<ErrorResponse>(error_response.into())))

                }
            }
        } else {
            let error_response = ErrorResponse {
                status: "error".to_string(),
                message: "You are not logged in, please provide a token".to_string(),
            };
            ready(Err(ErrorUnauthorized::<ErrorResponse>(error_response.into())))

        }
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}
#[derive(Debug, Serialize, Deserialize)]



pub struct Username(pub String);