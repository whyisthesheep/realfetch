use sysinfo::{System, Disks};
use std::env;

#[allow(dead_code)]
#[derive(Debug)]
struct FetchData {
    user: String,
    host: String,
    os: String,
    arch: String,
    uptime: String,
    cpu: String,
    cores: String,
    threads: String,
    memory: String,
    memory_cap: String,
    disk: String,
    disk_cap: String,
    disk_usage: String,
    local_ip: String,
}

trait PlatOp {
    fn fetch_data() -> FetchData;
}

#[cfg(target_os = "windows")]
struct Win;

#[cfg(target_os = "windows")]
impl PlatOp for Win {
    fn fetch_data() -> FetchData {
        let mut sys = System::new_all();
        sys.refresh_all();
        let osinf = os_info::get();

        let (disk_used, disk_total, disk_usage) = get_drive();

        FetchData {
            user: whoami::username(),
            host: whoami::fallible::hostname().unwrap_or("Windows".to_string()),
            os: format!("{} {} {} ({})", osinf.os_type(), osinf.edition().unwrap_or(""), osinf.version(), osinf.bitness()),
            arch: env::consts::ARCH.to_string(),
            uptime: format_uptime(sysinfo::System::uptime()),
            cpu: sys.cpus()[0].brand().trim().to_string(),
            cores: num_cpus::get_physical().to_string(),
            threads: num_cpus::get().to_string(),
            memory: format!("{:.1} GB", sys.used_memory() as f64 / 1073741824.0), //1024 cubed
            memory_cap: format!("{:.1} GB", sys.total_memory() as f64 / 1073741824.0),
            disk: format!("{:.1} GB", disk_used),
            disk_cap: format!("{:.1} GB", disk_total),
            disk_usage: format!("{:.1}%", disk_usage),
            local_ip: local_ip_address::local_ip().unwrap().to_string(),
        }
    }
}

#[cfg(target_os = "windows")]
fn get_drive() -> (f64, f64, f64) {
    let disks = Disks::new_with_refreshed_list();
    if let Some(disk) = disks.list().first() {
        let used_space_b = disk.total_space() - disk.available_space();
        let total_space_b = disk.total_space();

        const GB: f64 = 1024.0 * 1024.0 * 1024.0;
        let used_gb = used_space_b as f64 / GB;
        let total_gb = total_space_b as f64 / GB;

        let percent_used = if total_gb > 0.0 {
            (used_gb as f64 / total_gb as f64) * 100.0
        } else {
            0.0
        };

        return (used_gb, total_gb, percent_used);
    } else {
        eprintln!("No disk???");
        return (1 as f64, 1 as f64, 1 as f64);
        //this is the worst error handling ever
    }
}

#[cfg(target_os = "windows")]
fn format_uptime(seconds: u64) -> String {
    let days = seconds / (24 * 3600); 
    let hours = (seconds % (24 * 3600)) / 3600; //yes this could be more concise but I dont care
    let minutes = (seconds % 3600) / 60;
    format!("{}d {}h {}m", days, hours, minutes)
}

fn printfetch(x: FetchData) {
    let userhost = format!("{}@{}", x.user, x.host);
    let mut bar = "".to_string();
    (0..=userhost.len()).into_iter().for_each(|_| {
        bar.push('=');
    });
    bar.pop();

    println!(
        "\n{}@{}\n{}\n\n| OS: {}\n| Architecture: {}\n| Uptime: {}\n| CPU: {}\n| CPU Cores: {}, Threads: {}\n| RAM: {}/{}\n| Disk: {}/{} (Usage {})\n| Local IP: {}\n",
        x.user, x.host, bar, x.os, x.arch, x.uptime, x.cpu, x.cores, x.threads, x.memory, x.memory_cap, x.disk, x.disk_cap, x.disk_usage, x.local_ip
    );
}

#[cfg(target_os = "windows")]
fn main() {
    printfetch(Win::fetch_data());
}