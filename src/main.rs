mod data;
use data::fetch;

#[derive(Debug)]
struct FetchData {
    user: String,
    host: String,
    os: String,
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
    let (ds, dc, du) = fetch::get_drive();
    let (us, hs, op, up, ip) = fetch::get_sys();
    let (ps, co, tr, me, mc) = fetch::get_hardware();
    
    FetchData {
        user: us,
        host: hs,
        os: op,
        uptime: up,
        cpu: ps,
        cores: co,
        threads: tr,
        memory: me,
        memory_cap: mc,
        disk: ds,
        disk_cap: dc,
        disk_usage: du,
        local_ip: ip,
    }
}

fn printfetch(x: FetchData) {
    let userhost = format!("{}@{}", x.user, x.host);
    let bar = "=".repeat(userhost.len());
    
    println!(
        "\n{userhost}\n\
        {bar}\n\n\
         | OS: {os}\n\
         | Uptime: {uptime}\n\
         | CPU: {cpu}\n\
         | CPU Details: {cores} Cores, {threads} Threads\n\
         | RAM Usage: {memory}\n\
         | RAM: {memory_cap}\n\
         | Disk: {disk}/{disk_cap} (Usage {disk_usage})\n\
         | Local IP: {local_ip}\n",
        userhost = userhost,
        bar = bar,
        os = x.os,
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