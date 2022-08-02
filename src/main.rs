use colored::*;
use os_release::OsRelease;
use std::io;
use sysinfo::SystemExt;

fn main() {
    let mut sys = sysinfo::System::new();

    sys.refresh_all();

    let hostname = sys.host_name().unwrap();
    let uptime_hours = sys.uptime() as f64 / 3600.0;
    let uptime_minutes = (uptime_hours % 1.0) * 60.0;
    let kernel = sys.kernel_version().unwrap();
    let distro = get_distro().unwrap().name;

    if distro.to_lowercase().contains("arch") {
        eprintln!("{}", include_str!("ascii-arts/arch").bold().white());
    } else {
        println!("Distro currently not supported!");
    }

    println!(
        "Hostname: {}\n\
        Uptime: {}hrs, {}min\n\
        Kernel: {}\n\
        Distro: {}",
        hostname, uptime_hours as u64, uptime_minutes as u64, kernel, distro
    );
}

pub fn get_distro() -> Result<OsRelease, io::Error> {
    return OsRelease::new();
}
