use clap::Parser;
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Parser)]
#[command(version, about, author)]
struct Cli {
    /// Prints CPU usage once per second, forever
    #[arg(short, long)]
    watch: bool,

    /// Clear the screen after printing (watch-mode only)
    #[arg(short, long)]
    clear: bool,
}

fn main() {
    let args = Cli::parse();

    let mut sys = System::new_all();

    // Make CPU readings accurate
    for _ in 0..2 {
        std::thread::sleep(std::time::Duration::from_millis(250));
        sys.refresh_cpu();
    }

    if args.watch {
        loop {
            if args.clear {
                print!("{esc}c", esc = 27 as char);
            }
            print_cpu_usages(&mut sys);
        }
    } else {
        print_cpu_usages(&mut sys);
    }
}

/// Waits 1 second and then prints CPU usages
fn print_cpu_usages(sys: &mut System) {
    // Delay between readings, then refresh CPU
    std::thread::sleep(std::time::Duration::from_millis(1000));
    sys.refresh_cpu();

    // Get CPU usages
    let cpu_usages: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    // Format CPU usages
    let cpu_usage_strings: Vec<String> = cpu_usages
        .iter()
        .map(|cpu_usage| format!("{:>3.0}", cpu_usage))
        .collect();
    let formatted_cpu_usages = cpu_usage_strings.join(" ");

    // Output CPU usages
    println!("{}", formatted_cpu_usages);
}
