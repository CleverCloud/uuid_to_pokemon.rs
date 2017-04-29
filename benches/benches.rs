#![feature(test)]

extern crate test;
extern crate uuid;
extern crate uuid_to_pokemon;

use test::{Bencher, black_box};
use std::fmt::Write;
use uuid::Uuid;
use uuid_to_pokemon::uuid_to_pokemon;

#[bench]
fn bench_eq(b: &mut Bencher) {
    let u = Uuid::nil();
    b.iter(|| {
        for _ in 0..100 {
            black_box(uuid_to_pokemon(u) == "Busy bulbasaur");
            black_box("Busy bulbasaur" == uuid_to_pokemon(u));
        }
    });
}

#[bench]
fn bench_eq_rand(b: &mut Bencher) {
    b.iter(|| {
        let u = Uuid::new_v4();
        for _ in 0..100 {
            black_box(uuid_to_pokemon(u) == "Busy bulbasaur");
            black_box("Busy bulbasaur" == uuid_to_pokemon(u));
        }
    });
}

#[bench]
fn bench_write(b: &mut Bencher) {
    b.iter(|| {
        let mut s = String::new();
        let u = Uuid::new_v4();
        for _ in 0..100 {
            let _ = write!(s, "{}", uuid_to_pokemon(u));
        }
        s
    });
}
