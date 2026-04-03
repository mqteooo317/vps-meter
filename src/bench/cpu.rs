use aes::Aes256;
use cipher::{BlockEncrypt, KeyInit};
use cipher::generic_array::GenericArray;
use sha2::{Sha256, Digest};
use rayon::prelude::*;
use std::time::Instant;
use super::CpuResult;

pub fn run() -> CpuResult {
    let aes_gb_s = benchmark_aes();
    let sha_mb_s = benchmark_sha();
    let gflops = benchmark_matrix_mul();

    CpuResult {
        aes_gb_s,
        sha_mb_s,
        gflops,
    }
}

fn benchmark_aes() -> f64 {
    let key = GenericArray::from([0u8; 32]);
    let cipher = Aes256::new(&key);
    let mut blocks: Vec<GenericArray<u8, _>> = (0..1024)
        .map(|_| GenericArray::from([0u8; 16]))
        .collect();

    let start = Instant::now();
    for block in blocks.iter_mut() {
        cipher.encrypt_block(block);
    }
    let elapsed = start.elapsed();

    let bytes_processed = (blocks.len() * 16) as f64;
    bytes_processed / elapsed.as_secs_f64() / 1024.0 / 1024.0 / 1024.0
}

fn benchmark_sha() -> f64 {
    let data = vec![0u8; 10 * 1024 * 1024];
    let start = Instant::now();
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let _result = hasher.finalize();
    let elapsed = start.elapsed();

    (data.len() as f64) / elapsed.as_secs_f64() / 1024.0 / 1024.0
}

fn fastrand_f32() -> f32 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    
    let seed = RandomState::new().build_hasher().finish();
    let x = (seed >> 32) as u32;
    let y = seed as u32;
    
    let mut z = (x ^ y) as u64;
    z = z ^ (z >> 12);
    z = z ^ (z << 25);
    z = z ^ (z >> 27);
    let bits = ((z * 0x2545F4914F6CDD1Du64) >> 32) as u32;
    
    (bits as f32) / (u32::MAX as f32)
}

fn benchmark_matrix_mul() -> f64 {
    let size = 512;
    let a: Vec<Vec<f32>> = (0..size).map(|_| (0..size).map(|_| fastrand_f32()).collect()).collect();
    let b: Vec<Vec<f32>> = (0..size).map(|_| (0..size).map(|_| fastrand_f32()).collect()).collect();

    let start = Instant::now();
    let _c = multiply_matrices(&a, &b);
    let elapsed = start.elapsed();

    let ops = 2.0 * (size as f64).powi(3);
    ops / elapsed.as_secs_f64() / 1e9
}

fn multiply_matrices(a: &[Vec<f32>], b: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let n = a.len();
    let mut c = vec![vec![0.0; n]; n];

    c.par_iter_mut().enumerate().for_each(|(i, row)| {
        for k in 0..n {
            let aik = a[i][k];
            if aik != 0.0 {
                for j in 0..n {
                    row[j] += aik * b[k][j];
                }
            }
        }
    });

    c
}