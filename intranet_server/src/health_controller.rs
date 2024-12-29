use actix_web::{get, HttpResponse};
use gethostname::gethostname;
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct PiHealth {
    host_name: String,
    used_mem: u64,
    total_mem: u64,
    used_mem_as_pct: f64,
}

impl PiHealth {
    pub fn new(host_name: String, used_mem: u64, total_mem: u64, used_mem_as_pct: f64) -> PiHealth {
        PiHealth {
            host_name,
            used_mem,
            total_mem,
            used_mem_as_pct,
        }
    }
}

#[get("/health")]
pub async fn get_pi_health() -> HttpResponse {
    println!("Getting server health");
    let system_info = System::new_all();
    let server_info = PiHealth::new(
        gethostname().into_string().unwrap(),
        system_info.used_memory(),
        system_info.total_memory(),
        (system_info.used_memory() as f64) / (system_info.total_memory() as f64) * 100.0,
    );
    HttpResponse::Ok().json(server_info)
}
