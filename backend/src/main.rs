use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};


use fordev::routes::user::configure_routes;
use fordev::services::db::connect_to_mongodb;



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


// The main entry point of the application
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = "127.0.0.1";
    const PORT: u16 = 8080;

    // Connect to MongoDB and obtain the client instance
    let db_connection = connect_to_mongodb()
        .await
        .expect("Failed to connect to MongoDB");

    // Create an HTTP server using Actix Web
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone())) // Share the MongoDB client across routes
            .service(hello)
            .configure(configure_routes)
    })
    .bind((server_address, PORT))?; // Bind the server to the specified address and port

    server.run().await // Start the server
}
