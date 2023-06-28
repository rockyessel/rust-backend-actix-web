use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::{self, doc, Document}, Client};
use serde_json::json;
use futures::stream::StreamExt;

use crate::models::user::User;
use crate::utils::helpers::hashed_or_verity_pass;

pub async fn create_user(
    web::Form(form): web::Form<User>, // Extract the form data from the request
    mongo_client: web::Data<Client>,  // Access the MongoDB client instance
) -> impl Responder {
    // Create a User instance from the form data
    let user: User = User {
        name: form.name.to_string(),
        username: form.username.to_string(),
        email: form.email.to_string(),
        bio: form.bio.to_string(),
        password: form.password.to_string(),
    };

    // Access the "rustBackendApp" database
    let db = mongo_client.database("rustBackendApp");
    // Access the "users" collection within the database
    let collection = db.collection::<Document>("users");

    let email_exists = collection
        .find_one(doc! {"email": &form.email}, None)
        .await
        .expect("Something went wrong in with the email")
        .is_some();

    if email_exists {
        let err_msg = json!({"status": "failed request", "message": "Email already exists"});
        return HttpResponse::Conflict()
            .content_type("application/json")
            .json(err_msg);
    }

    let username_exist = collection
        .find_one(doc! {"username": &form.username}, None)
        .await
        .expect("Failed to execute find_one operation")
        .is_some();

    if username_exist {
        let err_msg = json!({"status": "failed request", "message": "Username already exists"});
        return HttpResponse::Conflict()
            .content_type("application/json")
            .json(err_msg);
    }

    let hashed_pass = hashed_or_verity_pass("test123Q@.", &form.username, "create_hash",Some("") );
    // Convert the user data to a BSON document
    let user_doc = bson::to_document(&user).expect("Failed to convert user to BSON document");

    // Insert the user document into the collection
    if let Err(err) = collection.insert_one(user_doc, None).await {
        eprintln!("Failed to insert user: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    let user_json = json!(user).to_string();

    // Return an HTTP response with the message
    HttpResponse::Ok()
        .content_type("application/json")
        .body(user_json)
}


pub async fn get_all_users(mongo_client: web::Data<Client>) -> impl Responder {
    // Access the "rustBackendApp" database
    let db = mongo_client.database("rustBackendApp");
    // Access the "users" collection within the database
    let collection = db.collection::<Document>("users");

    if let Ok(cursor) = collection.find(None, None).await {
        // Iterate over the documents returned by the cursor
        let users: Vec<User> = cursor
            .filter_map(|doc_result| async { doc_result.ok()?.try_into().ok() })
            .collect()
            .await;

        // Return an HTTP response with the users data as JSON
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(users);
    }

    // If an error occurred, return an internal server error response
    HttpResponse::InternalServerError().finish()
}

// pub async fn user_logout(web::Form(form): web::Form<User>,mongo_client: web::Data<Client>) -> impl Responder {
    
    
    
//     HttpResponse::InternalServerError().finish()
// }