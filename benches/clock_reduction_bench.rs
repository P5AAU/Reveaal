use std::sync::{Arc, Mutex};

use criterion::{criterion_group, criterion_main, Criterion};
use reveaal::ComponentLoader;

mod bench_helper;
use reveaal::extract_system_rep::create_executable_query;
use reveaal::parse_queries::parse_to_query;

const QUERY: &str = "refinement: (((((Adm2 && HalfAdm1 && HalfAdm2) || Machine || Researcher) && ((Adm2 && HalfAdm1) || Machine || Researcher) && ((Adm2 && HalfAdm2) || Machine || Researcher) && ((HalfAdm1 && HalfAdm2) || Machine || Researcher) && (Adm2 || Machine || Researcher)) // (Adm2 && HalfAdm1 && HalfAdm2)) // Researcher) <= (((((Adm2 && HalfAdm1 && HalfAdm2) || Machine || Researcher) && ((Adm2 && HalfAdm1) || Machine || Researcher) && ((Adm2 && HalfAdm2) || Machine || Researcher) && ((HalfAdm1 && HalfAdm2) || Machine || Researcher) && (Adm2 || Machine || Researcher)) // (Adm2 && HalfAdm1 && HalfAdm2)) // Researcher)";

/// This bench runs `QUERY` with and without clock reduction such that you can compare the results.
/// The bench takes about 40 min on my machine, so grab some coffee.
fn bench_clock_reduced_refinement(c: &mut Criterion) {
    // Set up the bench.
    let mut group = c.benchmark_group("Clock Reduction");
    group.bench_function("Refinement check - No reduction", |b| {
        let mut loader = bench_helper::get_uni_loader();
        loader.get_settings_mut().disable_clock_reduction = true;
        let loader = &Arc::new(Mutex::new(loader));
        b.iter(|| normal_refinement(Arc::clone(loader)));
    });
    group.bench_function("Refinement check - With reduction", |b| {
        let mut loader = bench_helper::get_uni_loader();
        loader.get_settings_mut().disable_clock_reduction = false;
        b.iter(|| clock_reduced_refinement(Arc::new(Mutex::new(loader))));
    });
    group.finish();
}

fn clock_reduced_refinement(loader: Arc<Mutex<dyn ComponentLoader>>) {
    let query = parse_to_query(QUERY);
    create_executable_query(query.get(0).unwrap(), loader)
        .unwrap()
        .execute();
}

fn normal_refinement(loader: Arc<Mutex<dyn ComponentLoader>>) {
    let query = parse_to_query(QUERY);
    create_executable_query(query.get(0).unwrap(), loader)
        .unwrap()
        .execute();
}

criterion_group! {
    name = clock_reduction_bench;
    config = Criterion::default().sample_size(10);
    targets = bench_clock_reduced_refinement
}
criterion_main!(clock_reduction_bench);
