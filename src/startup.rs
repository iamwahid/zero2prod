use std::net::TcpListener;

use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::{PgPool};

use crate::routes::health_check;
use crate::routes::subscribe;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}