use colored::*;
use os_release::OsRelease;
use std::io;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use whoami;

const ARCH_BASED_DISTROS: [&str; 2] = ["arch", "arcolinux"];

fn is_distro_arch_based(distroid: &&str) -> bool {
    if ARCH_BASED_DISTROS.contains(&&*distroid) {
        true
    } else {
        false
    }
}

fn main() {
    let divider: ColoredString = "--------------------".bold().green();
    let mut sys = System::new_all();
    sys.refresh_all();

    let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
    let uptime_hours = sys.uptime() as f64 / 3600.0;
    let uptime_minutes = (uptime_hours % 1.0) * 60.0;
    let kernel = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let distro = get_distro().unwrap().name;
    let distroid = get_distro().unwrap().id;
    let username = get_username();
    let cpu_name = sys.global_cpu_info().brand().to_string();
    let core_count = sys.physical_core_count().unwrap_or(1);
    let memory_used = sys.used_memory() / 1024; // MB
    let memory_total = sys.total_memory() / 1024; // MB
    let disk_used = get_used_disk_space(&sys) / 1024 / 1024 / 1024; // GB
    let disk_total = get_total_disk_space(&sys) / 1024 / 1024 / 1024; // GB

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
        "{}: {}@{}",
        "User".bold().blue(),
        username.bold(),
        hostname.bold()
    );
    println!("{}", divider);
    println!(
        "{:<10}: {} hrs, {} mins",
        "Uptime".bold().blue(),
        uptime_hours as u64,
        uptime_minutes as u64
    );
    println!(
        "{:<10}: {}",
        "Kernel".bold().blue(),
        kernel.bold()
    );
    println!(
        "{:<10}: {}",
        "Distro".bold().blue(),
        distro.bold()
    );
    println!(
        "{:<10}: {} ({} cores)",
        "CPU".bold().blue(),
        cpu_name,
        core_count
    );
    println!(
        "{:<10}: {} MB / {} MB",
        "Memory".bold().blue(),
        memory_used,
        memory_total
    );
    println!(
        "{:<10}: {} GB / {} GB",
        "Disk".bold().blue(),
        disk_used,
        disk_total
    );
}

pub fn get_distro() -> Result<OsRelease, io::Error> {
    OsRelease::new()
}

pub fn get_username() -> String {
    whoami::username()
}

pub fn get_total_disk_space(sys: &System) -> u64 {
    sys.disks()
        .iter()
        .map(|disk | disk.total_space())
        .sum()
}

pub fn get_used_disk_space(sys: &System) -> u64 {
    sys.disks()
        .iter()
        .map(|disk| disk.total_space() - disk.available_space())
        .sum()
}