mod data;
use data::fetch;

fn main() {
    let (us, hs, op, up, ip) = fetch::get_sys();
    let (ps, co, tr, me, mc) = fetch::get_hardware();
    let (ds, dc, du) = fetch::get_drive();


    let userhost = format!("{}@{}", us, hs);
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
        os = op,
        uptime = up,
        cpu = ps,
        cores = co,
        threads = tr,
        memory = me,
        memory_cap = mc,
        disk = ds,
        disk_cap = dc,
        disk_usage = du,
        local_ip = ip,
    );
}