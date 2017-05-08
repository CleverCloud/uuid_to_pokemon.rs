#![feature(test)]

extern crate test;
extern crate uuid;
extern crate uuid_to_pokemon;

use test::Bencher;
use std::fmt::Write;
use uuid::Uuid;
use uuid_to_pokemon::uuid_to_pokemon;

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
