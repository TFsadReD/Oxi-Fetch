use colored::*;

use crate::info::SystemInfo;
use crate::logos;


pub fn display(info: &SystemInfo) {
    render_top_header();
    render_system_details(info);
}

fn render_top_header() {
    let top_logo = r#"
  /$$$$$$  /$$   /$$ /$$$$$$       /$$$$$$$$ /$$$$$$$$ /$$$$$$$$ /$$$$$$  /$$   /$$
 /$$__  $$| $$  / $$|_  $$_/      | $$_____/| $$_____/|__  $$__//$$__  $$| $$  | $$
| $$  \ $$|  $$/ $$/  | $$        | $$      | $$         | $$  | $$  \__/| $$  | $$
| $$  | $$ \  $$$$/   | $$ /$$$$$$| $$$$$   | $$$$$      | $$  | $$      | $$$$$$$$
| $$  | $$  >$$  $$   | $$|______/| $$__/   | $$__/      | $$  | $$      | $$__  $$
| $$  | $$ /$$/\  $$  | $$        | $$      | $$         | $$  | $$    $$| $$  | $$
|  $$$$$$/| $$  \ $$ /$$$$$$      | $$      | $$$$$$$$   | $$  |  $$$$$$/| $$  | $$
 \______/ |__/  |__/|______/      |__/      |________/   |__/   \______/ |__/  |__/
    "#;
    println!("{}", top_logo.bright_green().bold());
}

fn render_system_details(info: &SystemInfo) {
    let logo = logos::get_os_logo(&info.os_id);

    let mut info_lines = vec![
        format!("{}@{}", info.user.bright_green().bold(), info.host.bright_green().bold()),
        "----------------------------------".dimmed().to_string(),
        format!("{}: {} {}", "OS".bright_yellow().bold(), info.os_name, info.os_version),
        format!("{}: {}", "Kernel".bright_yellow().bold(), info.kernel),
        format!("{}: {}h {}m", "Uptime".bright_yellow().bold(), info.uptime_hours, info.uptime_mins),
    ];

    for (key, value) in &info.extra_details {
        info_lines.push(format!("{}: {}", key.bright_yellow().bold(), value));
    }

    info_lines.push(format!("{}: {}", "CPU".bright_yellow().bold(), info.cpu_name));
    info_lines.push(format!("{}: {}", "GPU".bright_yellow().bold(), info.gpu_name));

    if !(info.gpu_usage.is_none()) {
        info_lines.push(format!("{}: {:.2?}% / {:.2?}%", "CPU/GPU Usage".bright_yellow().bold(), info.cpu_usage, info.gpu_usage));
    } else {
        info_lines.push(format!("{}: {:.2?}%", "CPU Usage".bright_yellow().bold(), info.cpu_usage));
    }

    info_lines.push(format!("{}: {} MiB / {} MiB", "Memory".bright_yellow().bold(), info.used_mem_mb, info.total_mem_mb));
    info_lines.push(format!("{}: {} Gb / {} Gb", "Disk:".bright_yellow().bold(), info.disk_used_gb, info.disk_total_gb));
    info_lines.push(format!("{}: {}", "Local IP".bright_yellow().bold(), info.net_ip));
    info_lines.push(format!(
            "{}",
            "███".black().to_string()
                + &"███".red().to_string()
                + &"███".green().to_string()
                + &"███".yellow().to_string()
                + &"███".blue().to_string()
                + &"███".magenta().to_string()
                + &"███".cyan().to_string()
                + &"███".white().to_string()
        ));
    info_lines.push(format!(
            "{}",
            "███".bright_black().to_string()
                + &"███".bright_red().to_string()
                + &"███".bright_green().to_string()
                + &"███".bright_yellow().to_string()
                + &"███".bright_blue().to_string()
                + &"███".bright_magenta().to_string()
                + &"███".bright_cyan().to_string()
                + &"███".bright_white().to_string()
        ));

    let max_rows = logo.lines.len().max(info_lines.len());
    let empty_logo_padding = " ".repeat(logo.width);

    for i in 0..max_rows {
        // Если логотип закончился раньше, чем список info_lines, выводим блок из logo.width пробелов
        let logo_part = logo.lines.get(i).map(|s| s.as_str()).unwrap_or(&empty_logo_padding);
        let info_part = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        println!("{}  {}", logo_part, info_part);
    }
}
