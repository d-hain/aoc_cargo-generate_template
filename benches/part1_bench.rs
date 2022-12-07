use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};

fn part1_bench(c: &mut Criterion){
    let input = std::fs::read_to_string("../input.txt").unwrap();
    
    c.bench_function(
        "part1",
        |b| b.iter(|| rust::part1(&input))
    );
}

criterion_group!(benches, part1_bench);
criterion_main!(benches);
