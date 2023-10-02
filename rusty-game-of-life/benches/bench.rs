use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn universe_ticks(c: &mut Criterion) {
    let mut universe = rusty_game_of_life::Universe::new(256, 256, |i| {
        if i % 2 == 0 || i % 7 == 0 {
            rusty_game_of_life::Cell::Alive
        } else {
            rusty_game_of_life::Cell::Dead
        }
    });
    c.bench_function("Universe::tick", |b| b.iter(|| universe.tick()));
}

criterion_group!(benches, universe_ticks);
criterion_main!(benches);
