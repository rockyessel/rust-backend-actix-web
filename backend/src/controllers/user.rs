use crate::{models::user::User, utils::helpers::gen_jwt_tok};
use crate::utils::helpers::hashed_or_verity_pass;
use actix_web::{web, HttpResponse, Responder};
use futures::stream::StreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct ID {
    // Serializes as a hex string in all formats
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    oid: ObjectId,
}

pub async fn create_user(web::Form(form): web::Form<User>, mongo_client: web::Data<Client>) -> impl Responder {
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

    let hashed_pass =
        hashed_or_verity_pass(&form.password, &form.username, "create_hash", Some(""));

    // Create a User instance from the form data

    let id = ID {
        oid: ObjectId::new(),
    };

    let user: User = User {
        _id: Some(id.oid.to_hex()),
        name: None,
        username: form.username.to_string(),
        email: Some(form.email.expect("failed").to_string()),
        bio: None,
        password: hashed_pass.to_string(),
    };

    // Convert the user data to a BSON document
    let user_doc = bson::to_document(&user).expect("Failed to convert user to BSON document");

    // Insert the user document into the collection
    if let Err(err) = collection.insert_one(user_doc, None).await {
        eprintln!("Failed to insert user: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    let tok = gen_jwt_tok(&user.username).unwrap();

    let user_json = json!({
        "_id": &user._id,
        "name": &user.name,
        "username": &user.username,
        "email": &user.email,
        "bio": &user.bio,
        "token": tok,
    })
    .to_string();

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

pub async fn user_login(web::Form(form): web::Form<User>, mongo_client: web::Data<Client>) -> impl Responder {
    // Access the "rustBackendApp" database
    let db = mongo_client.database("rustBackendApp");
    // Access the "users" collection within the database
    let collection: Collection<User> = db.collection::<User>("users");

    let username_exist = collection
        .find_one(doc! {"username": &form.username}, None)
        .await
        .expect("No user exist with the username provided.");

    let user_doc = username_exist.unwrap();

    let verify_pass = hashed_or_verity_pass(
        &form.password,
        &form.username,
        "verify_hash",
        Some(&user_doc.password.as_ref()),
    );
 let tok = gen_jwt_tok(&user_doc.username).unwrap();
    if verify_pass == "verified" {
           let user_json = json!({
        "_id": user_doc._id,
        "name": user_doc.name,
        "username": user_doc.username,
        "email": user_doc.email,
        "bio": user_doc.bio,
        "token": tok,
    })
    .to_string();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(user_json)
    } else {
        let user_json = json!({ "msg": &verify_pass }).to_string();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(user_json)
    }
}
