use crate::controllers::lib_information::{get_package,get_packages,add_package};
use actix_web::web;

pub fn package_configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("v1/api/packages")
            .route("", web::get().to(get_package))
            .route("/all", web::get().to(get_packages))
            .route("", web::post().to(add_package))
    )
}