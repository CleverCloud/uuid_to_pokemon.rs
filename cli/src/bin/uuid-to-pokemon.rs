extern crate uuid;
extern crate uuid_to_pokemon;

use std::{env, process};
use uuid::Uuid;
use uuid_to_pokemon::uuid_to_pokemon;

fn main() {
    let mut uuids: Vec<Uuid> = vec![];
    let mut errors: Vec<String> = vec![];

    for arg in env::args().skip(1) {
        match Uuid::parse_str(&arg) {
            Ok(u) => uuids.push(u),
            Err(_) => errors.push(arg),
        }
    }

    if uuids.len() == 0 && errors.len() == 0 {
        uuids.push(Uuid::new_v4());
    }

    if errors.len() > 0 {
        for e in errors {
            eprintln!("{} is not a valid UUID", e);
        }
        process::exit(1);
    }

    for u in uuids {
        println!("{}\t{}", u, uuid_to_pokemon(u));
    }
}
