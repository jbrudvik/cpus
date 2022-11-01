use clap::Parser;
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Parser)]
#[command(version, about, author)]
struct Cli {}

/*
TODO: Figure out how to poll at an interval with bash (to start)
TODO: Add a -w / --watch flag that keeps this open (and keeps repeating every second or so)
TODO: Ensure readings are correct -- maybe sleep to figure out?
- Play with numbers to make it different
*/

fn main() {
    let mut sys = System::new_all();

    // Make CPU readings accurate
    for _ in 0..2 {
        std::thread::sleep(std::time::Duration::from_millis(250));
        sys.refresh_cpu();
    }

    let cpu_usages: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    let cpu_usage_strings: Vec<String> = cpu_usages
        .iter()
        .map(|cpu_usage| format!("{:.0}", cpu_usage))
        .collect();
    let output = cpu_usage_strings.join(" ");
    println!("{}", output);
}
