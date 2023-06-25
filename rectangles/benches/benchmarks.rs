#![feature(test)]
extern crate rectangles;
extern crate test;

use rectangles::{count, count_without_collecting};
use test::Bencher;

#[rustfmt::skip]
const ONE_RECTANGLE: &[&str; 3] = &[
    "+-+",
    "| |",
    "+-+",
];

const COMPLICATED_INPUT: &[&str; 5] = &[
    "+------+----+",
    "|      |    |",
    "+---+--+    |",
    "|   |       |",
    "+---+-------+",
];

#[rustfmt::skip]
const LARGE_INPUT: &[&str; 8] = &[
    "+---+--+----+",
    "|   +--+----+",
    "+---+--+    |",
    "|   +--+----+",
    "+---+--+--+-+",
    "+---+--+--+-+",
    "+------+  | |",
    "          +-+",
];

#[bench]
fn bench_test_large_input_with_vec(b: &mut Bencher) {
    b.iter(|| count(LARGE_INPUT))
}

#[bench]
fn bench_test_large_input_with_iter(b: &mut Bencher) {
    b.iter(|| count_without_collecting(LARGE_INPUT))
}

#[bench]
fn bench_test_small_input_with_vec(b: &mut Bencher) {
    b.iter(|| count(ONE_RECTANGLE))
}

#[bench]
fn bench_test_small_input_with_iter(b: &mut Bencher) {
    b.iter(|| count_without_collecting(ONE_RECTANGLE))
}

#[bench]
fn bench_test_complicated_input_with_vec(b: &mut Bencher) {
    b.iter(|| count(COMPLICATED_INPUT))
}

#[bench]
fn bench_test_complicated_input_with_iter(b: &mut Bencher) {
    b.iter(|| count_without_collecting(COMPLICATED_INPUT))
}
