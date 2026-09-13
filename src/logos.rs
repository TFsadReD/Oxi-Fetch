use colored::*;

const WINDOWS: &str = include_str!("logos/win11.txt");
const UBUNTU: &str = include_str!("logos/ubuntu.txt");
const DEBIAN: &str = include_str!("logos/debian.txt");
const DEFAULT: &str = include_str!("logos/default.txt");
const CACHYOS: &str = include_str!("logos/cachyos.txt");
const MINT: &str = include_str!("logos/mint.txt");
const MX: &str = include_str!("logos/mx.txt");
const MANJARO: &str = include_str!("logos/manjaro.txt");
const FEDORA: &str = include_str!("logos/fedora.txt");
const ARCH: &str = include_str!("logos/arch.txt");
const NIXOS: &str = include_str!("logos/nixos.txt");

pub struct Logo {
    pub lines: Vec<String>,
    pub width: usize,
}

pub fn get_os_logo(os_id: &str) -> Logo {
    let raw_ascii = match os_id {
        "windows" => WINDOWS,
        "ubuntu" => UBUNTU,
        "debian" => DEBIAN,
        "cachyos" => CACHYOS,
        "mint" => MINT,
        "mx" => MX,
        "manjaro" => MANJARO,
        "fedora" => FEDORA,
        "arch" => ARCH,
        "nixos" => NIXOS,
        _ => DEFAULT,
    };

    let raw_lines: Vec<&str> = raw_ascii.lines().collect();
    let width = raw_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    let colored_lines = raw_lines
        .into_iter()
        .map(|line| {
            let len = line.chars().count();
            let padded_line = format!("{}{}", line, " ".repeat(width.saturating_sub(len)));

            match os_id {
                "windows" => padded_line.bright_blue().bold().to_string(),
                "ubuntu" => padded_line.bright_red().bold().to_string(),
                "debian" => padded_line.red().bold().to_string(),
                "cachyos" => padded_line.cyan().bold().to_string(),
                "mint" => padded_line.bright_green().bold().to_string(),
                "mx" => padded_line.bright_black().bold().to_string(),
                "manjaro" => padded_line.green().bold().to_string(),
                "fedora" => padded_line.bright_cyan().bold().to_string(),
                "arch" => padded_line.cyan().bold().to_string(),
                "nixos" => padded_line.bright_cyan().bold().to_string(),
                _ => padded_line.yellow().to_string(),
            }
        })
        .collect();

    Logo {
        lines: colored_lines,
        width,
    }
}
