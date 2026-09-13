use std::process::Command;

pub fn detect_os_id(_os_name: &str) -> String {
    "windows".to_string()
}

pub fn get_gpu_info() -> (String, Option<f32>) {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name",
        ])
        .output();

    let mut gpu_name = "Unknown GPU".to_string();
    if let Ok(out) = output {
        let name = String::from_utf8_lossy(&out.stdout)
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
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
    if let Ok(comspec) = std::env::var("ComSpec") {
        let shell = comspec.split('\\').last().unwrap_or(&comspec).to_string();
        details.push(("Shell".to_string(), shell));
    }
    details
}