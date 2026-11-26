mod benchmark;
mod tests;

fn init_benchmark_list() -> benchmark::BenchmarkList {
    let mut benchmark_list = benchmark::BenchmarkList::new();
    benchmark_list.expand(tests::surface::test_list());
    benchmark_list
}
fn main() {
    let mut b = init_benchmark_list();
    b.run("surface", 100);
    b.run("sprite", 100);
}