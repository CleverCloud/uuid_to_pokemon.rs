# UUID to POKéMON

This simple crate allows you to translate an uuid to a pokémon name.

The purpose is to provide simple names to address objects, so you can easily talk with
people about the objects.

The function is surjective. This means several UUIDs will give the same name. We don't
consider it an issue since context (like owner of object) will help dedup the search.

## Usage

```rust
extern crate uuid;
extern crate uuid_to_pokemon;

use uuid::Uuid;
use uuid_to_pokemon::uuid_to_pokemon;

fn main() {
   let u = Uuid::new_v4();
   println!(uuid_to_pokemon(u)); // e.g. "Careless pidgeotto"
}
```
