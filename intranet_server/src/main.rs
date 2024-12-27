use actix_files as fs;
use actix_web::{App, HttpServer};
use gethostname::gethostname;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut host_name = gethostname().into_string().unwrap();

    if !host_name.contains("local") {
        host_name = format!("{}.local", host_name);
    }

    HttpServer::new(|| App::new().service(fs::Files::new("/", "./static").index_file("index.html")))
        .bind((host_name, 12000))?
        .run()
        .await
}
