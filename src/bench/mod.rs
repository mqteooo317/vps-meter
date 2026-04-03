pub mod cpu;
pub mod ram;
pub mod disk;
pub mod net;

pub struct CpuResult {
    pub aes_gb_s: f64,
    pub sha_mb_s: f64,
    pub gflops: f64,
}

pub struct RamResult {
    pub latency_ns: f64,
    pub throughput_gb_s: f64,
    pub available_bytes: u64,
}

pub struct DiskResult {
    pub seq_mb_s: f64,
    pub random_iops: f64,
    pub used_bytes: u64,
}

pub struct NetResult {
    pub latency_ms: f64,
    pub download_mbps: f64,
    pub max_mbps: u64,
}