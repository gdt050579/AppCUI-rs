mod benchmark;
mod tests;
mod utils;
mod params;

use params::Params;
use benchmark::BenchmarkList;

fn init_benchmark_list() -> BenchmarkList {
    let mut benchmark_list = BenchmarkList::new();
    benchmark_list.expand(tests::surface::test_list());
    benchmark_list.expand(tests::desktop::test_list());
    benchmark_list.expand(tests::overlapping_windows::test_list());
    benchmark_list
}
fn list_benchmarks(params: &Params, lst: &BenchmarkList) {
    for benchmark in lst.list() {
        if benchmark.name().to_lowercase().contains(&params.filter) {
            println!("{:20} - {}", benchmark.name(), benchmark.description());
        }
    }
}
fn run_benchmark(params: &Params, lst: &BenchmarkList) {

}
fn show_help() {
    println!("Usage: benchmark command [arguments]");
    println!("Where command is one of the following:");
    println!("  help - Show this help message");
    println!("  list - List all available benchmarks");
    println!("  run  - Run a benchmark");
    println!();
    println!("Arguments:");
    println!("  --filter <text> - a text to filter the list of benchmarks");
    println!("  --times <count> - The number of times to run the benchmark (default: 10)");
}
fn main() {
    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);
    for arg in &mut args {
        arg.make_ascii_lowercase();
    }
    if args.is_empty() {
        show_help();
        return;
    }
    let params = match Params::new(&args) {
        Ok(params) => params,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };
    let benchmark_list = init_benchmark_list();
    match args[0].as_str() {
        "help" => show_help(),
        "list" => list_benchmarks(&params, &benchmark_list),
        "run" => run_benchmark(&params, &benchmark_list),
        _ => show_help(),
    }
}   