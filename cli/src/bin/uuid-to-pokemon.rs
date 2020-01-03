extern crate uuid;
extern crate uuid_to_pokemon;
extern crate structopt;
extern crate structopt_derive;

use uuid::Uuid;
use uuid_to_pokemon::uuid_to_pokemon;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "UUID to POKEMON")]
struct Opt {
    #[structopt(help = "List of UUIDs")]
    uuids: Vec<Uuid>,
}

fn main() {
    let mut opt = Opt::from_args();

    if opt.uuids.len() == 0 {
        opt.uuids.push(Uuid::new_v4());
    }

    for u in opt.uuids {
        println!("{}\t{}", u, uuid_to_pokemon(u));
    }
}
