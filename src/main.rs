use clap::Parser;
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Parser)]
#[command(version, about, author)]
struct Cli {
    /// Prints CPU usage once per second, forever
    #[arg(short, long)]
    watch: bool,
}

fn main() {
    let args = Cli::parse();

    let mut sys = System::new_all();

    if args.watch {
        loop {
            print_cpu_usages(&mut sys);
        }
    } else {
        print_cpu_usages(&mut sys);
    }
}

type CpuUsages = Vec<f32>;

fn get_cpu_usages(sys: &mut System) -> CpuUsages {
    // Make CPU readings accurate
    for _ in 0..4 {
        std::thread::sleep(std::time::Duration::from_millis(250));
        sys.refresh_cpu();
    }

    sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
}

fn format_cpu_usages(cpu_usages: CpuUsages) -> String {
    let cpu_usage_strings: Vec<String> = cpu_usages
        .iter()
        .map(|cpu_usage| format!("{:>3.0}", cpu_usage))
        .collect();
    cpu_usage_strings.join(" ")
}

fn print_cpu_usages(sys: &mut System) {
    let cpu_usages = get_cpu_usages(sys);
    let formatted_cpu_usages = format_cpu_usages(cpu_usages);
    println!("{}", formatted_cpu_usages);
}
