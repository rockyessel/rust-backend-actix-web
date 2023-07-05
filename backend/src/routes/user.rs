// use crate::controllers::user::{create_user, get_all_users, user_login,update_user};
use actix_web::web;

use crate::controllers::user::{create_user, get_all_users, user_login, update_user};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/api/users")
            .route("", web::get().to(get_all_users))
            .route("/register", web::post().to(create_user))
            .route("/login", web::post().to(user_login))
            .route("/{username}", web::put().to(update_user))
    );
}
