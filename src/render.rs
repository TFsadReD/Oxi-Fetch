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
    println!("{}", top_logo.bright_cyan().bold());
}

fn render_system_details(info: &SystemInfo) {
    let logo = logos::get_os_logo(&info.os_id);

    let info_lines = vec![
        format!("{}@{}", info.user.bright_green().bold(), info.host.bright_green().bold()),
        "----------------------------------".dimmed().to_string(),
        format!("{}: {} {}", "OS".bright_yellow().bold(), info.os_name, info.os_version),
        format!("{}: {}", "Kernel".bright_yellow().bold(), info.kernel),
        format!("{}: {}h {}m", "Uptime".bright_yellow().bold(), info.uptime_hours, info.uptime_mins),
        format!("{}: {}", "CPU".bright_yellow().bold(), info.cpu_name),
        format!("{}: {} MiB / {} MiB", "Memory".bright_yellow().bold(), info.used_mem_mb, info.total_mem_mb),
        format!(
            "{}",
            "███".black().to_string()
                + &"███".red().to_string()
                + &"███".green().to_string()
                + &"███".yellow().to_string()
                + &"███".blue().to_string()
                + &"███".magenta().to_string()
                + &"███".cyan().to_string()
                + &"███".white().to_string()
        ),
        format!(
            "{}",
            "███".bright_black().to_string()
                + &"███".bright_red().to_string()
                + &"███".bright_green().to_string()
                + &"███".bright_yellow().to_string()
                + &"███".bright_blue().to_string()
                + &"███".bright_magenta().to_string()
                + &"███".bright_cyan().to_string()
                + &"███".bright_white().to_string()
        ),
    ];

    let max_rows = logo.lines.len().max(info_lines.len());
    let empty_logo_padding = " ".repeat(logo.width);

    for i in 0..max_rows {
        let logo_part = match logo.lines.get(i) {
            Some(line) => {
                let visible_len = line.chars().count();
                let padding = " ".repeat(logo.width.saturating_sub(visible_len));
                format!("{}{}", line, padding)
            }
            None => empty_logo_padding.clone(),
        };

        let info_part = info_lines.get(i).cloned().unwrap_or_default();

        println!("{}  {}", logo_part, info_part);
    }
}
