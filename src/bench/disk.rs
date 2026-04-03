use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::time::Instant;
use tempfile::tempdir;
use sysinfo::{System, Disks, Disk};
use super::DiskResult;

pub fn run() -> DiskResult {
    let seq_mb_s = benchmark_sequential();
    let random_iops = benchmark_random_4k();

    let mut sys = System::new();
    sys.refresh_memory();
    
    let disks = Disks::new_with_refreshed_list();
    let used_bytes: u64 = disks.iter().map(|d: &Disk| d.total_space() - d.available_space()).sum();

    DiskResult {
        seq_mb_s,
        random_iops,
        used_bytes,
    }
}

fn benchmark_sequential() -> f64 {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.dat");
    let size = 256 * 1024 * 1024;
    let data = vec![0u8; 64 * 1024];

    let mut file = File::create(&file_path).unwrap();
    let start = Instant::now();
    let mut written = 0;
    while written < size {
        written += file.write(&data).unwrap();
    }
    let elapsed = start.elapsed();

    let _ = std::fs::remove_file(file_path);
    (size as f64) / elapsed.as_secs_f64() / 1024.0 / 1024.0
}

fn benchmark_random_4k() -> f64 {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.dat");
    let size = 50 * 1024 * 1024;
    let data = vec![0u8; 4096];

    {
        let mut file = File::create(&file_path).unwrap();
        for _ in 0..(size / 4096) {
            file.write_all(&data).unwrap();
        }
    }

    let mut file = OpenOptions::new().read(true).open(&file_path).unwrap();
    let start = Instant::now();
    let iterations = 5000;

    for i in 0..iterations {
        let offset = (i * 4096) % (size - 4096);
        file.seek(SeekFrom::Start(offset as u64)).unwrap();
        let mut buf = [0u8; 4096];
        file.read_exact(&mut buf).unwrap();
    }

    let elapsed = start.elapsed();
    let _ = std::fs::remove_file(file_path);

    iterations as f64 / elapsed.as_secs_f64()
}