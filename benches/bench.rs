use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn universe_ticks(c: &mut Criterion) {
    let mut universe = wasm_game_of_life::Universe::new();
    c.bench_function("Universe::tick", |b| b.iter(|| universe.tick()));
}

criterion_group!(benches, universe_ticks);
criterion_main!(benches);

