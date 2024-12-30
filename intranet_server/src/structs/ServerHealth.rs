use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ServerHealth {
    host_name: String,
    used_mem: u64,
    total_mem: u64,
    used_mem_as_pct: Option<u64>,
}

impl ServerHealth {
    pub fn new(host_name: string, used_mem: u64, total_mem: u64) -> ServerHealth {
        ServerHealth {
            host_name,
            used_mem,
            total_mem,
            used_mem_as_pct: None,
        }
    }

    pub fn calc_mem_pct(&self) -> f64 {
        (&self.used_mem as f64) / (&self.total_mem as f64) * 100
    }
}
