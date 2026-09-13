use std::thread;
use sysinfo::{Disks, Networks, System, MINIMUM_CPU_UPDATE_INTERVAL};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
mod fallback {
    pub fn detect_os_id(os_name: &str) -> String {
        os_name.to_lowercase()
    }
    pub fn get_gpu_info() -> (String, Option<f32>) {
        ("Unknown GPU".to_string(), None)
    }
    pub fn get_extra_info() -> Vec<(String, String)> {
        vec![]
    }
}

#[cfg(target_os = "linux")]
use linux as platform;
#[cfg(target_os = "windows")]
use windows as platform;
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
use fallback as platform;

pub struct SystemInfo {
    pub user: String,
    pub host: String,
    pub os_name: String,
    pub os_id: String,
    pub os_version: String,
    pub kernel: String,
    pub uptime_hours: u64,
    pub uptime_mins: u64,
    pub cpu_name: String,
    pub cpu_usage: f32,
    pub gpu_name: String,
    pub gpu_usage: Option<f32>,
    pub net_ip: String,
    pub disk_used_gb: u64,
    pub disk_total_gb: u64,
    pub used_mem_mb: u64,
    pub total_mem_mb: u64,
    pub extra_details: Vec<(String, String)>,
}

impl SystemInfo {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_usage();

        let user = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "user".to_string());

        let host = System::host_name().unwrap_or_else(|| "localhost".to_string());
        let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
        let os_version = System::os_version().unwrap_or_default();
        let kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

        let uptime_secs = System::uptime();
        let uptime_hours = uptime_secs / 3600;
        let uptime_mins = (uptime_secs % 3600) / 60;

        let cpu_name = sys
            .cpus()
            .first()
            .map(|cpu| cpu.brand().trim().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());

        // Безопасный подсчет средней загрузки CPU по всем ядрам
        let cpus = sys.cpus();
        let cpu_usage = if !cpus.is_empty() {
            cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
        } else {
            0.0
        };

        let total_mem_mb = sys.total_memory() / (1024 * 1024);
        let used_mem_mb = sys.used_memory() / (1024 * 1024);

        // Инициализация дисков без использования new_with_refreshed
        let mut disks = Disks::new();
        disks.refresh(true);
        let (disk_used_gb, disk_total_gb) = disks
            .iter()
            .next()
            .map(|disk| {
                let total = disk.total_space() / (1024 * 1024 * 1024);
                let available = disk.available_space() / (1024 * 1024 * 1024);
                (total - available, total)
            })
            .unwrap_or((0, 0));

        // Инициализация сети без использования new_with_refreshed
        let mut networks = Networks::new();
        networks.refresh(true);
        let net_ip = networks
            .iter()
            .find_map(|(_, data)| {
                data.ip_networks()
                    .iter()
                    .find(|ip| !ip.addr.is_loopback() && ip.addr.is_ipv4())
                    .map(|ip| ip.addr.to_string())
            })
            .unwrap_or_else(|| "Offline".to_string());

        let os_id = platform::detect_os_id(&os_name);
        let (gpu_name, gpu_usage) = platform::get_gpu_info();
        let extra_details = platform::get_extra_info();

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
            cpu_usage,
            gpu_name,
            gpu_usage,
            net_ip,
            disk_used_gb,
            disk_total_gb,
            used_mem_mb,
            total_mem_mb,
            extra_details,
        }
    }
}