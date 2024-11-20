use colored::*;
use os_release::OsRelease;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use whoami;

const ARCH_BASED_DISTROS: [&str; 2] = ["arch", "arcolinux"];

fn is_distro_arch_based(distroid: &str) -> bool {
    ARCH_BASED_DISTROS.contains(&distroid)
}

fn main() {
    let divider: ColoredString = "--------------------".bold().green();

    let mut sys = System::new_all();
    sys.refresh_all();

    let hostname = sys.host_name().unwrap_or_else(|| "Unknown".to_string());
    let kernel = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let cpu_name = sys.global_cpu_info().brand().to_string();
    let core_count = sys.physical_core_count().unwrap_or(1);
    let memory_used = sys.used_memory() / 1024; // MB
    let memory_total = sys.total_memory() / 1024; // MB
    let disk_used = get_used_disk_space(&sys) / 1024 / 1024 / 1024; // GB
    let disk_total = get_total_disk_space(&sys) / 1024 / 1024 / 1024; // GB
    let uptime_hours = sys.uptime() / 3600;
    let uptime_minutes = (sys.uptime() % 3600) / 60;

    let distro_info = get_distro();
    let distro_name = distro_info.name;
    let distro_id = distro_info.id;

    let username = whoami::username();

    match distro_id.as_str() {
        id if is_distro_arch_based(id) => eprintln!("{}", include_str!("ascii-arts/arch").bold().green()),
        "ubuntu" => eprintln!("{}", include_str!("ascii-arts/ubuntu").bold().red()),
        "manjaro" => eprintln!("{}", include_str!("ascii-arts/manjaro").bold().green()),
        _ => println!("Distro currently not supported!"),
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
        uptime_hours,
        uptime_minutes
    );
    println!(
        "{:<10}: {}",
        "Kernel".bold().blue(),
        kernel.bold()
    );
    println!(
        "{:<10}: {}",
        "Distro".bold().blue(),
        distro_name.bold()
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

fn get_distro() -> OsRelease {
    OsRelease::new().unwrap_or_else(|_| OsRelease {
        name: "Unknown".to_string(),
        id: "unknown".to_string(),
        pretty_name: "Unknown".to_string(),
        version: "Unknown".to_string(),
        ..Default::default()
    })
}

fn get_total_disk_space(sys: &System) -> u64 {
    sys.disks().iter().map(|disk| disk.total_space()).sum()
}

fn get_used_disk_space(sys: &System) -> u64 {
    sys.disks()
        .iter()
        .map(|disk| disk.total_space() - disk.available_space())
        .sum()
}