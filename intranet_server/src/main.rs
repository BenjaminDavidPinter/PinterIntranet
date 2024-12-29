mod health_controller;

use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use gethostname::gethostname;
use health_controller::get_pi_health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let mut host_name = gethostname().into_string().unwrap();

    if !host_name.contains("local") {
        host_name = format!("{}.local", host_name);
    }

    println!("Starting server on {}:12000", host_name);

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").service(get_pi_health))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind((host_name, 12000))?
    .run()
    .await
}
