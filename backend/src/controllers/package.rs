use actix_web::{web, HttpResponse, Responder};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    Client, Collection,
};
use crate::{models::package::Package};

pub async fn get_package() {}
pub async fn get_packages() {}
pub async fn add_package(web::Form(form):web::Form<Package>, mongo_client:web::Data<Client>) -> impl Responder {
        // Access the "rustBackendApp" database
    let db = mongo_client.database("rustBackendApp");
    // Access the "users" collection within the database
    let collection = db.collection::<Document>("users");

}
pub async fn update_lib_info() {}
pub async fn delete_lib_info() {}