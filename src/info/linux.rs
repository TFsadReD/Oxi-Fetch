use std::fs;
use std::process::Command;

pub fn detect_os_id(os_name: &str) -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(id) = line.strip_prefix("ID=") {
                return id.trim_matches('"').to_lowercase();
            }
        }
    }
    os_name.to_lowercase()
}

pub fn get_gpu_info() -> (String, Option<f32>) {
    let mut gpu_name = "Unknown GPU".to_string();

    if let Ok(out) = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d' | cut -d ':' -f3")
        .output()
    {
        let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !name.is_empty() {
            gpu_name = name;
        }
    }

    let gpu_usage = Command::new("nvidia-smi")
        .args(["--query-gpu=utilization.gpu", "--format=csv,noheader,nounits"])
        .output()
        .ok()
        .and_then(|out| {
            String::from_utf8_lossy(&out.stdout)
                .trim()
                .parse::<f32>()
                .ok()
        });

    (gpu_name, gpu_usage)
}

pub fn get_extra_info() -> Vec<(String, String)> {
    let mut details = Vec::new();
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        details.push(("DE/WM".to_string(), desktop));
    }
    if let Ok(shell) = std::env::var("SHELL") {
        let shell_name = shell.split('/').last().unwrap_or(&shell).to_string();
        details.push(("Shell".to_string(), shell_name));
    }
    details
}