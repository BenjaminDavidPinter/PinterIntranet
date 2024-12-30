use actix_web::{get, HttpResponse};
use gethostname::gethostname;
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
struct SystemHealth {
    host_name: String,
    used_mem: u64,
    total_mem: u64,
    used_mem_as_pct: f64,
    cpu_usages_as_pct: Vec<f32>,
}

impl SystemHealth {
    pub fn new(
        host_name: String,
        used_mem: u64,
        total_mem: u64,
        used_mem_as_pct: f64,
        cpu_usages_as_pct: Vec<f32>,
    ) -> SystemHealth {
        SystemHealth {
            host_name,
            used_mem,
            total_mem,
            used_mem_as_pct,
            cpu_usages_as_pct,
        }
    }
}

#[get("/health")]
pub async fn get_pi_health() -> HttpResponse {
    println!("Getting server health");
    let mut system_info = System::new_all();

    let mut cpus: Vec<f32> = Vec::new();

    //In order for sysinfo to take accurate
    //usage measurements, we have to call this
    //function again
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    system_info.refresh_cpu_usage();

    for cpu in system_info.cpus() {
        cpus.push(cpu.cpu_usage());
    }

    let server_info = SystemHealth::new(
        gethostname().into_string().unwrap(),
        system_info.used_memory(),
        system_info.total_memory(),
        (system_info.used_memory() as f64) / (system_info.total_memory() as f64) * 100.0,
        cpus,
    );
    HttpResponse::Ok().json(server_info)
}
