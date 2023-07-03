use argon2::{Config, ThreadMode, Variant, Version};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest;
use scraper::{self, Element, Html, Selector};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Deserialize, Serialize)]
pub struct JWTUserClaims {
    pub username: String,
    pub exp: usize,
}

// https://pypi.org/project/json2json/
async fn get_html() -> Result<String, reqwest::Error> {
    let response = reqwest::get("http://localhost:3000/get-html?url=https://yarnpkg.com/package/aws-amplify").await?;
    let html_content = response.text().await?;
    Ok(html_content)
}

pub async fn gen_data_from_html_link() -> Option<String> {
    match get_html().await {
        Ok(content) => {
            let h2_value = scrape_h2_value(&content);
            println!("H2 Value: {}", h2_value);
            Some(h2_value)
        }
        Err(error) => {
            println!("Error: {}", error);
            None
        }
    }
}

fn scrape_h2_value(content: &str) -> String {
    let document = Html::parse_fragment(content);

    println!("{:?}", &document);

    let selector = Selector::parse("h1#aws-amplify-package---aws-amplify").unwrap();

    let h1 = document.select(&selector).next();

    let text: String = h1.unwrap().text().collect();

    println!("{:#?}", text);

    if let Some(h2_element) = document.select(&selector).next() {
        h2_element.text().collect()
    } else {
        String::new()
    }
}
pub fn hashed_or_verity_pass(
    password: &str,
    username: &str,
    method: &str,
    db_hashed_pass: Option<&str>,
) -> String {
    const PEPPER: &[u8] = b"yn9j6u1Tjzaho75c";
    const SECRET: &[u8] = b"QGMWgFK2x_@_awxZEHzdWV";
    let salt = "1c3e5d9f7b8a6c2e".as_bytes();

    let config = Config {
        variant: Variant::Argon2i,
        version: Version::Version13,
        mem_cost: 65536, // memory cost (kiB)
        time_cost: 4,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: &SECRET,
        ad: username.as_bytes(),
        hash_length: 32,
    };

    let mut strong_pass = Vec::new();
    strong_pass.extend_from_slice(PEPPER);
    strong_pass.extend_from_slice(password.as_bytes());
    strong_pass.extend_from_slice(&salt);

    match method {
        "create_hash" => {
            let hash = argon2::hash_encoded(&strong_pass, &salt, &config).unwrap();
            hash
        }
        "verify_hash" => match db_hashed_pass {
            Some(db_hashed_pass) => {
                let verified_hash: bool = argon2::verify_encoded_ext(
                    &db_hashed_pass,
                    &strong_pass,
                    &SECRET,
                    username.as_bytes(),
                )
                .unwrap();
                match verified_hash {
                    true => "verified".to_string(),
                    _ => "not verified".to_string(),
                }
            }
            None => "No hash provided".to_string(),
        },
        _ => "Invalid method".to_string(),
    }
}

pub fn gen_jwt_tok(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let current_time = Utc::now();
    let expiration_time = current_time + Duration::days(7);
    let user_claim = JWTUserClaims {
        username: username.to_string(),
        exp: expiration_time.timestamp() as usize,
    };

    let secret_key = "your-secret-key".as_bytes();
    let encoding_key = EncodingKey::from_secret(secret_key);
    let header = Header::default();

    let tok = encode(&header, &user_claim, &encoding_key);
    tok
}
