#[macro_use]
extern crate diesel;

#[macro_use]
extern crate log;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

use std::env;

mod jwt;
mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match jwt::validate_token(credentials.token()).await {
        Ok(user) => {
            // req.get_session().set("user", user);
            Ok(req)
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let app_host = env::var("APP_HOST").expect("APP_HOST not found.");
    let app_port = env::var("APP_PORT").expect("APP_PORT not found.");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);

        App::new()
            .data(pool.clone())
            .service(
                web::scope("/users")
                .wrap(auth)
                .route("", web::get().to(handlers::get_users))
                .route("/{id}", web::get().to(handlers::get_user_by_id))
                .route("", web::post().to(handlers::add_user))
                .route("/{id}", web::delete().to(handlers::delete_user))
            )
            .route("/login", web::post().to(handlers::login))
            
    })
    .bind(&format!("{}:{}", &app_host, &app_port))?
    .run()
    .await
}
