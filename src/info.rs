use std::fs;
use sysinfo::System;

pub struct SystemInfo {
    pub user: String,
    pub host: String,
    pub os_name: String,
    pub os_id: String,
    pub os_version: String,
    pub kernel: String,
    pub uptime_hours: usize,
    pub uptime_mins: usize,
    pub cpu_name: String,
    pub used_mem_mb: usize,
    pub total_mem_mb: usize,
}


impl SystemInfo {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let user = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "user".to_string());

        let host = System::host_name().unwrap_or_else(|| "localhost".to_string());
        let os_name = System::name().unwrap_or_else(|| "Linux".to_string());
        let os_version = System::os_version().unwrap_or_default();
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

        let uptime_secs = System::uptime();
        let uptime_hours = (uptime_secs / 3600) as usize;
        let uptime_mins = ((uptime_secs % 3600) / 60) as usize;

        let total_mem_mb = (sys.total_memory() / (1024 * 1024)) as usize;
        let used_mem_mb = (sys.used_memory() / (1024 * 1024)) as usize;

        let cpu_name = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().trim().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        let os_id = Self::detect_os_id().unwrap_or_else(|| os_name.to_lowercase());

        Self {
            user,
            host,
            os_name,
            os_id,
            os_version,
            kernel,
            uptime_hours,
            uptime_mins,
            cpu_name,
            used_mem_mb,
            total_mem_mb,
        }
    }

    fn detect_os_id() -> Option<String> {
        let content = fs::read_to_string("/etc/os-release").ok()?;
        for line in content.lines() {
            if let Some(id) = line.strip_prefix("ID=") {
                return Some(id.trim_matches('"').to_lowercase());
            }
        }
        None
    }
}
