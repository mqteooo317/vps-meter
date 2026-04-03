use sysinfo::{System, Cpu};
use std::process::Command;

pub struct HardwareInfo {
    pub hostname: String,
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub ram_total: u64,
    pub disk_type: String,
}

pub fn detect() -> HardwareInfo {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    sys.refresh_memory();

    let hostname = get_hostname();
    let cpu_model = get_cpu_model(&sys);
    let cpu_cores = get_cpu_cores(&sys);
    let ram_total = sys.total_memory();
    let disk_type = detect_disk_type();

    HardwareInfo {
        hostname,
        cpu_model,
        cpu_cores,
        ram_total,
        disk_type,
    }
}

fn get_hostname() -> String {
    Command::new("hostname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "unknown".to_string())
        .trim()
        .to_string()
}

fn get_cpu_model(sys: &System) -> String {
    let from_sysinfo = sys.cpus()
        .first()
        .map(|cpu: &Cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "".to_string());
    
    if !from_sysinfo.is_empty() && !from_sysinfo.contains("CPU") {
        return from_sysinfo;
    }
    
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") || line.starts_with("Processor") {
                if let Some(model) = line.split(':').nth(1) {
                    let model = model.trim().to_string();
                    if !model.is_empty() {
                        return model;
                    }
                }
            }
        }
    }
    
    "Unknown CPU".to_string()
}

fn get_cpu_cores(sys: &System) -> usize {
    let from_sysinfo = sys.cpus().len();
    
    if from_sysinfo > 0 {
        return from_sysinfo;
    }
    
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        let count = content.lines()
            .filter(|line| line.starts_with("processor"))
            .count();
        if count > 0 {
            return count;
        }
    }
    
    if let Ok(output) = Command::new("nproc").output() {
        if let Ok(s) = String::from_utf8(output.stdout) {
            if let Ok(cores) = s.trim().parse::<usize>() {
                return cores;
            }
        }
    }
    
    1
}

fn detect_disk_type() -> String {
    if let Ok(output) = Command::new("lsblk")
        .arg("-d")
        .arg("-o")
        .arg("ROTA")
        .arg("-n")
        .arg("--noheadings")
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("0") {
            return "SSD/NVMe".to_string();
        }
    }
    
    if let Ok(output) = Command::new("df").arg("-T").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("f2fs") || output_str.contains("ext4") {
            return "Flash Storage".to_string();
        }
    }
    
    "HDD/Unknown".to_string()
}