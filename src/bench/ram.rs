use std::ptr;
use std::time::Instant;
use sysinfo::System;
use super::RamResult;

pub fn run() -> RamResult {
    let latency_ns = benchmark_latency();
    let throughput_gb_s = benchmark_throughput();
    let mut sys = System::new();
    sys.refresh_memory();
    let available_bytes = sys.available_memory();

    RamResult {
        latency_ns,
        throughput_gb_s: if throughput_gb_s.is_finite() { throughput_gb_s } else { 0.0 },
        available_bytes,
    }
}

fn benchmark_latency() -> f64 {
    let size = 64 * 1024 * 1024;
    let vec: Vec<u8> = vec![0; size];
    let ptr = vec.as_ptr();
    let iterations = 5_000_000;

    let start = Instant::now();
    for i in 0..iterations {
        let offset = (i * 4096) % size;
        unsafe {
            ptr::read_volatile(ptr.add(offset));
        }
    }
    let elapsed = start.elapsed();

    elapsed.as_secs_f64() / iterations as f64 * 1e9
}

fn benchmark_throughput() -> f64 {
    let size = 50 * 1024 * 1024;
    let src: Vec<u8> = vec![1; size];
    let mut dst: Vec<u8> = vec![0; size];

    let start = Instant::now();
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), size);
    }
    let elapsed = start.elapsed();

    (size as f64) / elapsed.as_secs_f64() / 1024.0 / 1024.0 / 1024.0
}