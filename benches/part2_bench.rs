use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};

fn part2_bench(c: &mut Criterion){
    let input = std::fs::read_to_string("../input.txt").unwrap();
    
    c.bench_function(
        "part2",
        |b| b.iter(|| rust::part2(&input))
    );
}

criterion_group!(benches, part2_bench);
criterion_main!(benches);
