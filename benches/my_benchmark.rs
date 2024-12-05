use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

use advent_of_code_2024::day3;
use advent_of_code_2024::day4;

pub fn day3(criterion: &mut Criterion) {
    let input = include_str!("./input-3.txt");
    criterion.bench_function("day3/part1", |b| b.iter(|| day3::part1(black_box(input))));
    criterion.bench_function("day3/part2", |b| b.iter(|| day3::part2(black_box(input))));
    let output = day3::part1(input);
    assert_eq!(output.to_string(), "169021493");
    let output = day3::part2(input);
    assert_eq!(output.to_string(), "111762583");
}

pub fn day4(criterion: &mut Criterion) {
    let input = read_to_string("./benches/input-4.txt").unwrap();
    let s = input.as_str();
    criterion.bench_function("day4/part1", |b| b.iter(|| day4::part1(black_box(s))));
    criterion.bench_function("day4/part2", |b| b.iter(|| day4::part2(black_box(s))));
    let output = day4::part1(s);
    assert_eq!(output.to_string(), "2397");
    let output = day4::part2(s);
    assert_eq!(output.to_string(), "1824");
}

criterion_group!(benches, day4);
criterion_main!(benches);
