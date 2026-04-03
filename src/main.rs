use clap::Parser;
use std::time::Instant;

mod bench;
mod hardware;
mod utils;
mod score;
mod recommendations;
mod output;
mod compare;

#[derive(Parser)]
#[command(name = "vps-meter")]
#[command(about = "VPS performance benchmarking tool", long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub json: bool,

    #[arg(short, long)]
    pub quiet: bool,

    #[arg(short, long)]
    pub compare: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if !cli.quiet && !cli.json {
        print_banner();
    }

    let hw = hardware::detect();

    if !cli.quiet && !cli.json {
        print_hardware_info(&hw);
        println!("\n\x1b[1;35m┌─────────────────────────────────────────────┐\x1b[0m");
        println!("\x1b[1;35m│                   WAIT...                   │\x1b[0m");
        println!("\x1b[1;35m└─────────────────────────────────────────────┘\x1b[0m\n");
    }

    let start = Instant::now();

    // Run benchmarks in parallel using rayon
    let (cpu_results, rest) = rayon::join(
        || bench::cpu::run(),
        || {
            rayon::join(
                || bench::ram::run(),
                || {
                    rayon::join(
                        || bench::disk::run(),
                        || bench::net::run()
                    )
                }
            )
        }
    );
    let (ram_results, rest2) = rest;
    let (disk_results, net_results) = rest2;

    let elapsed = start.elapsed();

    let scores = score::calculate_all(&cpu_results, &ram_results, &disk_results, &net_results);
    let recommendations = recommendations::generate(&cpu_results, &ram_results, &disk_results, &net_results, &scores);

    if let Some(baseline_path) = &cli.compare {
        if let Err(e) = compare::compare_with_baseline(
            &cpu_results, &disk_results, &net_results,
            &scores, baseline_path
        ) {
            eprintln!("\x1b[1;31mError loading baseline: {}\x1b[0m", e);
        }
    } else if cli.json {
        let json_output = output::to_json(&hw, &cpu_results, &ram_results, &disk_results, &net_results, &scores, &recommendations, elapsed);
        println!("{}", json_output);
    } else {
        output::to_console(&hw, &cpu_results, &ram_results, &disk_results, &net_results, &scores, &recommendations, elapsed, cli.verbose);
    }
}

fn print_banner() {
    println!("\n\x1b[1;36m┌─────────────────────────────────────────────┐\x1b[0m");
    println!("\x1b[1;36m│              VPS METER v0.4                 │\x1b[0m");
    println!("\x1b[1;36m├─────────────────────────────────────────────┤\x1b[0m");
    println!("\x1b[1;36m│        \x1b[36mmateoo.pages.dev/github              \x1b[1;36m│\x1b[0m");
    println!("\x1b[1;36m│        \x1b[36mmateoo.pages.dev/vps-meter           \x1b[1;36m│\x1b[0m");
    println!("\x1b[1;36m└─────────────────────────────────────────────┘\x1b[0m\n");
}

fn print_hardware_info(hw: &hardware::HardwareInfo) {
    println!("\x1b[1;33mHost:\x1b[0m {}", hw.hostname);
    println!("\x1b[1;33mCPU:\x1b[0m {} ({} cores)", hw.cpu_model, hw.cpu_cores);
    println!("\x1b[1;33mRAM:\x1b[0m {:.2} GB", hw.ram_total as f64 / 1024.0 / 1024.0 / 1024.0);
    println!("\x1b[1;33mDisk:\x1b[0m {}\n", hw.disk_type);
}