mod info;
mod render;
mod logos;

use info::SystemInfo;

fn main() {
    let sys_info = SystemInfo::collect();
    render::display(&sys_info);
}
