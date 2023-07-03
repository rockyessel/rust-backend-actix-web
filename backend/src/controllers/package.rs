use actix_web::{HttpResponse, Responder};
use serde_json;

use crate::utils::helpers::gen_data_from_html_link;

pub async fn add_package() -> impl Responder {
    let title = gen_data_from_html_link().await;

    let value = serde_json::json!({
        "code": 200,
        "success": true,
        "payload": {
            "features": [
                "serde",
                "json"
            ],
            "homepage": null,
            "title": title,
        }
    });

    let res = HttpResponse::Ok()
        .content_type("application/json")
        .json(value);
    res
}
