use crate::controllers::package::add_package;
use crate::middleware::authentication::JwtMiddleware;
use actix_web::web;

pub fn package_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("v1/api/packages")
            .route(web::post().to(add_package))
            .wrap(JwtMiddleware) // Add the JwtMiddleware here to protect the route
    );
}
