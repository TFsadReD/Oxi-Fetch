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
        _ => DEFAULT,
    };

    let lines: Vec<String> = raw_ascii.lines().map(|s| s.to_string()).collect();

    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    let colored_lines = lines
        .into_iter()
        .map(|line| match os_id {
            "windows" => line.bright_blue().bold().to_string(),
            "ubuntu" => line.bright_red().bold().to_string(),
            "debian" => line.red().bold().to_string(),
            "cachyos" => line.cyan().bold().to_string(),
            "mint" => line.bright_green().bold().to_string(),
            "mx" => line.bright_black().bold().to_string(),
            "manjaro" => line.green().bold().to_string(),
            "fedora" => line.bright_cyan().bold().to_string(),
            "arch" => line.cyan().bold().to_string(),
            _ => line.yellow().to_string(),
        })
        .collect();

    Logo {
        lines: colored_lines,
        width,
    }
}
