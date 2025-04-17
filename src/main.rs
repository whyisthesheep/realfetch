use sysinfo::{System, Disks};

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

fn fetch_data() -> FetchData {
    let mut sys = System::new_all();
    sys.refresh_all();
    let osinf = os_info::get();

    let (disk_used, disk_total, disk_usage) = get_drive();

    FetchData {
        user: whoami::username(),
        host: whoami::fallible::hostname().unwrap_or("Windows".to_string()),
        os: format!("{} {} {} ({})", osinf.os_type(), osinf.edition().unwrap_or(""), osinf.version(), osinf.bitness()),
        arch: whoami::arch().to_string(),
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

fn format_uptime(s: u64) -> String {
    format!(
        "{}d {}h {}m",
        s / (24 * 3600),
        (s % (24 * 3600)) / 3600,
        (s % 3600) / 60
    )
}

fn printfetch(x: FetchData) {
    let userhost = format!("{}@{}", x.user, x.host);
    let bar = "=".repeat(userhost.len());
    
    println!(
        "\n{userhost}\n{bar}\n\n\
         | OS: {os}\n\
         | Architecture: {arch}\n\
         | Uptime: {uptime}\n\
         | CPU: {cpu}\n\
         | CPU Cores: {cores}, Threads: {threads}\n\
         | RAM: {memory}/{memory_cap}\n\
         | Disk: {disk}/{disk_cap} (Usage {disk_usage})\n\
         | Local IP: {local_ip}\n",
        userhost = userhost,
        bar = bar,
        os = x.os,
        arch = x.arch,
        uptime = x.uptime,
        cpu = x.cpu,
        cores = x.cores,
        threads = x.threads,
        memory = x.memory,
        memory_cap = x.memory_cap,
        disk = x.disk,
        disk_cap = x.disk_cap,
        disk_usage = x.disk_usage,
        local_ip = x.local_ip
    );
}

fn main() {
    printfetch(fetch_data());
}