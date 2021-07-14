use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, PlotConfiguration, AxisScale};
use rust_radix_sort::{radix_sort, generate_vector};

// A function to benchmark the radix sort algorithm's performance compared to the rust library's
// std::sort() function with large vectors. Uses Criterion for comparisons and plotting graphs.
fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sort vs Radix Sort");
    // Plot the results in on a logarithmic graph.
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    // Currently benches vectors containing up to one million integers.
    let range = 0..6;
    let vecs = range.clone().map(|z| (z, generate_vector(z as u32))).collect::<Vec<_>>();

    // Benchmark radix sort for each vector one by one.
    for (index, vec) in vecs {
        let current = vec;
        group.bench_with_input(BenchmarkId::new("Radix Sort", index), &current,
                                |b, v| b.iter(|| radix_sort(v))).plot_config(plot_config.clone());
        group.bench_with_input(BenchmarkId::new("Sort", index), &current,
                                |b, v| b.iter(|| v.clone().sort())).plot_config(plot_config.clone());
    }
    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);