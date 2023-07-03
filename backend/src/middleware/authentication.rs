use actix_service::forward_ready;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform, Payload},
    error::ErrorUnauthorized,
    FromRequest, HttpRequest, HttpMessage, Error,
};
use futures::future::{ready, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
// use std::future::LocalBoxFuture;
use futures::future::LocalBoxFuture;

use crate::utils::helpers::JWTUserClaims;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Username(pub String);

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareMiddleware { service }))
    }
}

pub struct JwtMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
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
            let secret = DecodingKey::from_secret("your-secret-key".as_ref());
            let validation = Validation::default();

            match decode::<JWTUserClaims>(&token, &secret, &validation) {
                Ok(token_data) => {
                    let username = token_data.claims.username.to_string();
                    req.extensions_mut().insert(Username(username.clone()));
                    let fut = self.service.call(req);

                    Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    })
                }
                Err(_) => {
                    let error_response = ErrorResponse {
                        status: "error".to_string(),
                        message: "Invalid token".to_string(),
                    };
                    Box::pin(async move {
                        Err(ErrorUnauthorized::<ErrorResponse>(error_response.into()))
                    })
                }
            }
        } else {
            let error_response = ErrorResponse {
                status: "error".to_string(),
                message: "You are not logged in, please provide a token".to_string(),
            };
            Box::pin(async move {
                Err(ErrorUnauthorized::<ErrorResponse>(error_response.into()))
            })
        }
    }
}

impl FromRequest for JwtMiddleware {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(_req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(JwtMiddleware))
    }
}
