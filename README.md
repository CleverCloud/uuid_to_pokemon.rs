# UUID to POKéMON

[![Build Status](https://travis-ci.org/CleverCloud/uuid_to_pokemon.rs.svg?branch=master)](https://travis-ci.org/CleverCloud/uuid_to_pokemon.rs)
[![Build status](https://ci.appveyor.com/api/projects/status/wkquvy5qw2sy3lmc?svg=true)](https://ci.appveyor.com/project/Keruspe/uuid-to-pokemon-rs)
[![Coverage Status](https://coveralls.io/repos/github/CleverCloud/uuid_to_pokemon.rs/badge.svg?branch=master)](https://coveralls.io/github/CleverCloud/uuid_to_pokemon.rs?branch=master)
[Documentation](https://docs.rs/uuid_to_pokemon)

This simple crate allows you to translate a UUID to a pokémon name.

The purpose is to provide simple names to address objects, so you can easily talk with
people about the objects.

The function is not injective. This means several UUIDs will give the same name. We don't
consider it an issue since context (like owner of object) will help dedup the search.

## Usage

```rust
extern crate uuid;
extern crate uuid_to_pokemon;

use uuid::Uuid;
use uuid_to_pokemon::uuid_to_pokemon;

fn main() {
   let u = Uuid::new_v4();
   println!("{}", uuid_to_pokemon(u)); // e.g. "Careless pidgeotto"
}
```
