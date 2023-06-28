use actix_web::web;
use crate::controllers::user::{create_user,get_all_users };

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/")).route("/user", web::get().to(get_all_users));
    cfg.service(web::scope("/user").route("/register", web::post().to(create_user)));
}