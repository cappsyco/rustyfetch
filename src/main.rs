use colored::*;
use os_release::OsRelease;
use std::io;
use sysinfo::SystemExt;
use whoami;

const ARCH_BASED_DISTROS: [&str; 2] = ["arch", "arcolinux"];

fn is_distro_arch_based(distroid: &&str) -> bool {
    return if ARCH_BASED_DISTROS.contains(&&*distroid) {
        true
    } else {
        false
    };
}

fn main() {
    let divider: ColoredString = "--------------------".bold().green();
    let mut sys = sysinfo::System::new();

    sys.refresh_all();

    let hostname = sys.host_name().unwrap();
    let uptime_hours = sys.uptime() as f64 / 3600.0;
    let uptime_minutes = (uptime_hours % 1.0) * 60.0;
    let kernel = sys.kernel_version().unwrap();
    let distro = get_distro().unwrap().name;
    let distroid = get_distro().unwrap().id;
    let username = get_username();

    if is_distro_arch_based(&&*distroid) {
        eprintln!("{}", include_str!("ascii-arts/arch").bold().green());
    } else if distroid=="ubuntu" {
        eprintln!("{}", include_str!("ascii-arts/ubuntu").bold().red());
    } else if distroid=="manjaro" {
        eprintln!("{}", include_str!("ascii-arts/manjaro").bold().green());
    } else {
        println!("Distro currently not supported!");
    }

    println!(
        "{}@{}\n\
        {}\n\
        Uptime: {}hrs, {}min\n\
        Kernel: {}\n\
        Distro: {}",
        username, hostname, divider, uptime_hours as u64, uptime_minutes as u64, kernel, distro
    );
}

pub fn get_distro() -> Result<OsRelease, io::Error> {
    return OsRelease::new();
}

pub fn get_username() -> String {
    whoami::username()
}
