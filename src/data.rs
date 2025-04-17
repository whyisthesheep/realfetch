pub mod fetch {
    use sysinfo::{Disks, System};

    pub fn get_drive() -> (String, String, String) {
        let disk_error = 
            (String::from("Disk error"), String::from("Disk error"), String::from("Disk error"));
    
        let disks = Disks::new_with_refreshed_list();
        if let Some(disk) = disks.list().first() {
            const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    
            let used_space = disk.total_space() - disk.available_space();
            let total_space = disk.total_space();
    
            let used_gb = used_space as f64 / GB;
            let total_gb = total_space as f64 / GB;
            let percentage = (used_space as f64 / total_space as f64) * 100.0;
    
            (
                format!("{:.1} GB", used_gb),
                format!("{:.1} GB", total_gb),
                format!("{:.1}%", percentage)
            )
        } else {
            disk_error
        }
    }

    pub fn get_sys() -> (String, String, String, String, String) {
        let mut sys = System::new_all();
        let osinf = os_info::get();
        sys.refresh_all();

        (
            whoami::username(),
            whoami::fallible::hostname().unwrap_or("Generic".to_string()),
            format!("{} {} {} ({})", osinf.os_type(), osinf.edition().unwrap_or(""), osinf.version(), osinf.bitness()),
            format_uptime(sysinfo::System::uptime()),
            local_ip_address::local_ip().unwrap().to_string()
        )
    }

    pub fn get_hardware() -> (String, String, String, String, String) {
        let mut sys = System::new_all();
        sys.refresh_all();

        (
            sys.cpus()[0].brand().trim().to_string(),
            num_cpus::get_physical().to_string(),
            num_cpus::get().to_string(),
            format!("{:.1} GB", sys.used_memory() as f64 / 1073741824.0),
            format!("{:.1} GB", sys.total_memory() as f64 / 1073741824.0)
        )
    }

    fn format_uptime(s: u64) -> String {
        format!(
            "{}d {}h {}m",
            s / (24 * 3600),
            (s % (24 * 3600)) / 3600,
            (s % 3600) / 60
        )
    }
}