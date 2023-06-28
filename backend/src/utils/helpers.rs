use argon2::{Config, ThreadMode, Variant, Version};
use reqwest;
use scraper;
use std::str;

pub fn get_data_from_html_link() {
    let html_response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    let parsed_document = scraper::Html::parse_document(&html_response);

    let get_titles_response = scraper::Selector::parse("h3.lister-item-header>a").unwrap();

    let titles = parsed_document
        .select(&get_titles_response)
        .map(|x| x.inner_html());

    titles
        .zip(1..101)
        .for_each(|(item, number)| println!("{}. {}", number, item));
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
                let verified_hash: bool = argon2::verify_encoded_ext(&db_hashed_pass, &strong_pass, &SECRET,username.as_bytes()).unwrap();
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

