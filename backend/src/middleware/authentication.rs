use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage, HttpResponse};
use futures_util::future::{ok, Either, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;

// Define your own Claims struct based on the JWT payload
#[derive(Debug, PartialEq, Deserialize)]
struct Claims {
    // Add necessary fields here
    username: String,
}

// Define the Example trait
trait Example {
    type Ready;
}

// Implement the Example trait for the desired type
impl<T> Example for T {
    type Ready = Ready<Result<ServiceResponse<T>, Error>>;
}

pub struct JwtAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddlewareService { service })
    }
}

pub struct JwtAuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    S::Future: futures_util::future::Future<Output = Result<ServiceResponse<B>, Error>>,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = futures_util::future::Ready<Result<Self::Response, Self::Error>>;
   fn call(&self, req: ServiceRequest) -> Self::Future {
        // Get the Authorization header from the request
        let token = match req.headers().get(AUTHORIZATION) {
            Some(header_value) => {
                if let Ok(token) = header_value.to_str() {
                    // Remove the "Bearer " prefix from the token
                    Ok(token.trim_start_matches("Bearer ").to_owned())
                } else {
                    Err("Invalid token".to_string())
                }
            }
            None => Err("Invalid token".to_string()),
        };

        let fut = match decode_token(&token.unwrap()) {
            Ok(claims) => {
                println!("Authenticated user: {:?}", claims.username);

                // Attach the verified data to the request
                req.extensions_mut().insert(claims);

                // Call the inner service with the modified request
                Either::Left(self.service.call(req))
            }
            Err(_) => {
                let res = HttpResponse::Unauthorized().finish();
                Ready::try_into(ok(ServiceResponse::new(
                    req.into_parts().0,
                    res,
                )))
            }
        };

   
        let res = fut;
        fut
      
   
    }

   fn poll_ready(&self, context: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
    todo!()
}
}

// Verify and decode the JWT token
fn decode_token(token: &str) -> Result<Claims, String> {
    // Set your JWT secret key or PEM file path
    let secret = "your_secret_key";
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    match decode::<Claims>(token, &decoding_key, &Validation::default()) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => Err("Invalid token".to_string()),
    }
}
