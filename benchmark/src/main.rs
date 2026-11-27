mod benchmark;
mod tests;

fn init_benchmark_list() -> benchmark::BenchmarkList {
    let mut benchmark_list = benchmark::BenchmarkList::new();
    benchmark_list.expand(tests::surface::test_list());
    benchmark_list.expand(tests::window::test_list());
    benchmark_list
}
fn main() {
    let mut b = init_benchmark_list();
    //b.run("surface", 1000);
    //b.run("sprite", 1000);
    let results = b.run("Window",100);
    for result in results {
        println!("{:?}", result);
    }
}