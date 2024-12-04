use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent_of_code_2024::day3::part1;
use advent_of_code_2024::day3::part2;

pub fn day3(criterion: &mut Criterion) {
    let input = include_str!("./input-3.txt");
    criterion.bench_function("part1", |b| b.iter(|| part1(black_box(input))));
    criterion.bench_function("part2", |b| b.iter(|| part2(black_box(input))));
    let output = part1(input);
    assert_eq!(output.to_string(), "169021493");
    let output = part2(input);
    assert_eq!(output.to_string(), "111762583");
}

criterion_group!(benches, day3);
criterion_main!(benches);
