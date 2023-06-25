use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{
    bson::{self, doc, Document},
    options::ClientOptions,
    Client,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    name: String,
    age: i32,
}

#[get("/")]
async fn hello() -> impl Responder {
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

#[post("/user")]
async fn create_user(
    web::Form(form): web::Form<UserData>,
    mongo_client: web::Data<Client>,
) -> impl Responder {
    let user: UserData = UserData {
        name: form.name.to_string(),
        age: form.age,
    };

    let db = mongo_client.database("rustBackendApp");
    let collection = db.collection::<Document>("users");

    let user_doc = bson::to_document(&user).expect("Failed to convert user to BSON document");

    if let Err(err) = collection.insert_one(user_doc, None).await {
        eprintln!("Failed to insert user: {}", err);
        return HttpResponse::InternalServerError().finish();
    }

    let message = format!(
        "Hello, {}! I'm {} ages old.",
        form.name.to_uppercase(),
        form.age
    );
    if let Err(err) = writeln!(io::stdout(), "Form: {:?}", form) {
        eprintln!("Failed to write to stdout: {}", err);
    }
    io::stdout().flush().expect("Failed to flush stdout");
    HttpResponse::Ok()
        .content_type("application/json")
        .body(message)
}

async fn connect_to_mongodb() -> Result<Client, Box<dyn Error>> {
    let uri = "mongodb+srv://rr:rr@rrcluster.x8kvi0e.mongodb.net/?retryWrites=true&w=majority";
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1";
    const PORT: u16 = 8080;
    let client = connect_to_mongodb()
        .await
        .expect("Failed to connect to MongoDB");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(create_user)
    })
    .bind((server_address, PORT))?;

    server.run().await
}
