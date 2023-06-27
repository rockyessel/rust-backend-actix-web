use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::stream::StreamExt;

use mongodb::{
    bson::{self, doc, Document},
    options::ClientOptions,
    Client,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, Write};


// Define a struct to represent user data
#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    name: String,
    age: i32,
}

// Define a handler for the root ("/") route
#[get("/")]
async fn hello() -> impl Responder {
    // Return a simple HTML response
    HttpResponse::Ok().content_type("text/html").body(
        r#"
    <h1>Hello, welcome to our API endpoints</h1>
    <p>Browse through the list below.</p>
    <ul>
        <li>Users</li>
        <li>Packages/Library</li>
        <li>Information</li>
        <li>Others</li>
    </ul>
    "#,
    )
}

// Define a handler for the "/user" route to create a new user
#[post("/user")]
async fn create_user(
    web::Form(form): web::Form<UserData>, // Extract the form data from the request
    mongo_client: web::Data<Client>,      // Access the MongoDB client instance
) -> impl Responder {
    // Create a UserData instance from the form data
    let user: UserData = UserData {
        name: form.name.to_string(),
        age: form.age,
    };

    // Access the "rustBackendApp" database
    let db = mongo_client.database("rustBackendApp");
    // Access the "users" collection within the database
    let collection = db.collection::<Document>("users");

    // Convert the user data to a BSON document
    let user_doc = bson::to_document(&user).expect("Failed to convert user to BSON document");

    // Insert the user document into the collection
    if let Err(err) = collection.insert_one(user_doc, None).await {
        eprintln!("Failed to insert user: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    // Prepare a response message with the user's name and age
    let message = format!(
        "Hello, {}! I'm {} ages old.",
        form.name.to_uppercase(),
        form.age
    );

    // Write the form data to stdout (console)
    if let Err(err) = writeln!(io::stdout(), "Form: {:?}", form) {
        eprintln!("Failed to write to stdout: {}", err);
    }
    io::stdout().flush().expect("Failed to flush stdout");

    // Return an HTTP response with the message
    HttpResponse::Ok()
        .content_type("application/json")
        .body(message)
}

// Connect to MongoDB and return a Client instance
async fn connect_to_mongodb() -> Result<Client, Box<dyn Error>> {
    let uri = "mongodb+srv://rr:rr@rrcluster.x8kvi0e.mongodb.net/?retryWrites=true&w=majority";
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

// Implement conversion from UserData to a BSON Document
impl From<UserData> for Document {
    fn from(user: UserData) -> Self {
        bson::to_document(&user).expect("Failed to convert user to BSON document")
    }
}

// Implement conversion from BSON Document to UserData
impl TryFrom<Document> for UserData {
    type Error = bson::de::Error;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        bson::from_document(doc)
    }
}

// Define a handler for the "/users" route to get all users
#[get("/users")]
async fn get_all_users(mongo_client: web::Data<Client>) -> impl Responder {
    // Access the "rustBackendApp" database
    let db = mongo_client.database("rustBackendApp");
    // Access the "users" collection within the database
    let collection = db.collection::<Document>("users");

    if let Ok(cursor) = collection.find(None, None).await {
        // Iterate over the documents returned by the cursor
        let users: Vec<UserData> = cursor
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

// The main entry point of the application
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1";
    const PORT: u16 = 8080;

    // Connect to MongoDB and obtain the client instance
    let client = connect_to_mongodb()
        .await
        .expect("Failed to connect to MongoDB");

    // Create an HTTP server using Actix Web
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone())) // Share the MongoDB client across routes
            .service(hello)
            .service(create_user)
            .service(get_all_users)
    })
    .bind((server_address, PORT))?; // Bind the server to the specified address and port

    server.run().await // Start the server
}
