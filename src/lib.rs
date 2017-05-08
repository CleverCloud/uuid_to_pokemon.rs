//! This simple crate allows you to translate a *UUID* to a pokémon name. The purpose is to provide
//! simple names to address objects, so you can easily talk with people about the objects.
//!
//! The function is not injective. This means several *UUID*s will give the same name. We don't
//! consider it an issue since context (like owner of object) will help dedup the search.
//!
//! ## Examples
//! ```rust
//! extern crate uuid;
//! extern crate uuid_to_pokemon;
//!
//! use uuid::Uuid;
//! use uuid_to_pokemon::{uuid_to_pokemon, PokemonUuid};
//! use std::fmt::Write;
//!
//! fn main() {
//!     let my_uuid: Uuid = Uuid::nil();
//!     let result: PokemonUuid = uuid_to_pokemon(my_uuid);
//!
//!    // write it into a string:
//!    let mut s = String::new();
//!    write!(s, "uuid `{}` as pokemon: `{}`", my_uuid, result).unwrap();
//!    assert_eq!(s, "uuid `00000000-0000-0000-0000-000000000000` as pokemon: `Busy bulbasaur`");
//!
//!    // if you need a simple String, use the to_string function:
//!    let s = result.to_string();
//!    assert_eq!(s, "Busy bulbasaur");
//! }
//! ```

extern crate uuid;

mod pokemons;

use std::fmt;
use uuid::Uuid;
use pokemons::{POKEMONS, ADJECTIVES};


/// Represents the result of [uuid_to_pokemon](fn.uuid_to_pokemon.html). It is a small wrapper
/// allowing us to operate without allocations.
///
/// For examples see the [module doc](index.html)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct PokemonUuid {
    adj: &'static str,
    pok: &'static str,
}

impl From<Uuid> for PokemonUuid {
    fn from(uuid: Uuid) -> PokemonUuid {
        uuid_to_pokemon(uuid)
    }
}

impl From<(&'static str, &'static str)> for PokemonUuid {
    fn from(v: (&'static str, &'static str)) -> PokemonUuid {
        PokemonUuid {
            adj: v.0,
            pok: v.1,
        }
    }
}

impl fmt::Display for PokemonUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.adj, self.pok)
    }
}

fn get_digit_mult(uuid: Uuid, first_index: usize) -> usize {
    let bs = uuid.as_bytes();
    let s1 = bs.iter().skip(first_index).take(4);
    let s2 = bs.iter().skip(first_index + 4).take(4);
    s1.zip(s2)
        .map(|(v1, v2)| (*v1 as usize) * (*v2 as usize))
        .sum()
}

/// Converts an *Uuid* into a *PokemonUuid*.
///
/// For examples see the [module doc](index.html)
pub fn uuid_to_pokemon(uuid: Uuid) -> PokemonUuid {
    let adj_index = get_digit_mult(uuid, 0) % ADJECTIVES.len();
    let pok_index = get_digit_mult(uuid, 8) % POKEMONS.len();
    PokemonUuid {
        adj: ADJECTIVES[adj_index],
        pok: POKEMONS[pok_index],
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    fn test_many_times(nb: u32) -> usize {
        let mut arr = HashSet::new();
        for _ in 0..nb {
            arr.insert(uuid_to_pokemon(Uuid::new_v4()));
        }
        arr.len()
    }

    #[test]
    fn test_uniqueness_level() {
        assert!(test_many_times(1000) >= 970);
        assert!(test_many_times(100) >= 98);
        assert_eq!(test_many_times(10), 10);
    }

    fn test_no_random(uuid: Uuid) {
        let mut set = HashSet::new();
        for _ in 0..500 {
            set.insert(uuid_to_pokemon(uuid));
        }
        assert_eq!(set.len(), 1)
    }

    #[test]
    fn test_values_are_always_the_same() {
        for _ in 0..4 {
            test_no_random(Uuid::new_v4());
        }
    }

    #[test]
    fn test_uuid_to_pokemon() {
        let ids = &[
            "43654e77-0aa4-4551-b545-b0ec6f895f53",
            "ca5da036-8c9e-473d-9392-03be3e928a21",
            "ef822821-83ab-436b-83fc-44feb5d5bf78",
            "d4bc783b-162e-4e5c-9230-d60f00823373",
            "b1e980c2-8bd7-424f-b88a-cb73dfb94a1d",
            "eb0b8047-e61a-44ae-ba53-b0c53150b253",
            "86258d72-c02c-4bee-9a10-1f67f188a215",
            "5226e680-d5a0-46af-a5e3-f7c0423c4ca0",
            "74d38a4b-093d-4032-8ed1-6e7a90ec9584",
            "5759b17c-5dc1-4972-b933-697d01cf6687",
            "3190e6d1-4056-446c-ab73-4ea182c12773",
            "42a2eed8-5014-438a-a0d8-0cedea5ba7c9",
            "401f8355-3bda-4f86-bb60-0080ec7dd842",
            "9d121736-c474-465b-8312-c684a4f88317",
            "617da971-d9ff-47a6-95d5-316a14bc573c",
            "3a4f7d8c-2b91-4d2f-ba81-3978e24a3d6d",
            "7dd21060-a5f7-4413-af57-1b2035bd3a6b",
            "4bb6ac20-a1e9-4672-8dec-5fc3b950100b",
            "5061ffa6-74ce-49f7-ac54-c91ef20502f2",
            "549cbf29-8286-495d-a265-542fe392144e",
            "30d25cca-2985-4ddb-b65c-6a08fd79edbd",
            "296aa001-cfb2-48b1-90fe-1ac608f53c33",
            "d52f9bac-2220-4d78-a4fc-3db594061e70",
            "980be4e6-4b61-4a51-aefb-c0d2606447d6",
            "51e83cf5-99ca-4bf1-96ec-b42f4d865520",
            "9edfdc07-897e-4edb-af2e-f08efe2b9465",
            "1b51f9c7-2371-43cf-8fa8-500e4813eaf5",
            "3bafc1b5-9f37-4784-9d55-5909536cb645",
            "bf938f7e-3221-4670-8b28-6a4bd67bd7bc",
            "f8c8dfb7-5ae1-449d-823c-204042a2f920",
            "637073ea-1cda-42c1-98da-eed21893e528",
            "6c3d2b3a-40e5-4761-81a0-ea608f751001",
            "a12d98cf-8d7e-465a-8639-fc1234f574a9",
            "a598e868-a1ce-4a5c-9c88-59236066f978",
            "90dcdb4e-af67-4022-9c50-9e05ac6a72d1",
            "c106d5c5-b843-4770-9ae6-c689df40bff4",
            "1744b076-e520-4517-8ab0-b23f1c7c4075",
            "4c1d404f-3748-4746-b3c7-67e0c501bc9d",
            "b65e711f-f959-462e-86bd-ceb39b0a262e",
            "3deee69d-6936-4b63-af5f-d6f4ad29c62a",
            "c9bb99f0-b0be-4975-866f-a9e9e8973140",
            "318bfcfb-30de-4897-bdfe-5cbcb5f8499d",
            "51370cfb-8de7-433a-8202-0f6061a48f8b",
            "7833b278-558f-4c76-9332-d39f51395d68",
            "53e2ae10-009f-4626-8fa8-650b6fc7a577",
            "0d8c6ca3-88af-45f8-8c6c-d184a9fb4909",
            "58e7e1c4-2e94-4e49-82af-0f270d56b347",
            "7537fc2a-f998-4efa-8f35-8011a9c0f808",
            "9f53d9bf-50a7-4a95-9130-fce9cc478ed3",
            "4aa8e0a4-184a-448c-9f49-2474e251c85a",
        ];

        let names = &[
            ("Amusing", "furret"),
            ("Great", "mienshao"),
            ("Sloppy", "pidgeotto"),
            ("Endless", "skarmory"),
            ("Strict", "gothitelle"),
            ("Wonderful", "togetic"),
            ("Shining", "lugia"),
            ("Friendly", "wobbuffet"),
            ("Hardworking", "magnemite"),
            ("Glowing", "qwilfish"),
            ("Slimy", "shedinja"),
            ("Deep", "serperior"),
            ("Splendid", "simisage"),
            ("Starving", "sawsbuck"),
            ("Beautiful", "mr-mime"),
            ("Polite", "pidgey"),
            ("Huge", "dedenne"),
            ("Cranky", "floette-eternal"),
            ("Stinky", "tyrogue"),
            ("Truthful", "hypno"),
            ("Truthful", "ralts"),
            ("Dreadful", "mismagius"),
            ("Loyal", "ninetales"),
            ("Vast", "pawniard"),
            ("Happy", "manectric-mega"),
            ("Smelly", "jellicent"),
            ("Wonderful", "ampharos-mega"),
            ("Fussy", "exeggutor"),
            ("Cheerful", "jumpluff"),
            ("Amusing", "kyurem-black"),
            ("Busy", "electivire"),
            ("Truthful", "mantyke"),
            ("Beautiful", "arceus"),
            ("Tough", "poliwrath"),
            ("Stiff", "nidorino"),
            ("Loyal", "kirlia"),
            ("Fussy", "greninja"),
            ("Adorable", "swampert-mega"),
            ("Truthful", "beheeyem"),
            ("Sloppy", "kakuna"),
            ("Spiky", "regigigas"),
            ("Hard", "honchkrow"),
            ("Dull", "oddish"),
            ("Beautiful", "hydreigon"),
            ("Clumsy", "absol-mega"),
            ("Sizzling", "bergmite"),
            ("Deep", "azurill"),
            ("Cranky", "gurdurr"),
            ("Foul", "corphish"),
            ("Crazy", "shroomish"),
        ];

        for (u, n) in ids.iter().zip(names.iter().cloned()) {
            assert_eq!(uuid_to_pokemon(Uuid::parse_str(u).unwrap()), n.into());
        }
    }

    #[test]
    fn test_uuid_to_pokemon_nil() {
        let u = Uuid::nil();
        assert_eq!(uuid_to_pokemon(u), ("Busy", "bulbasaur").into());
    }

    #[test]
    fn test_from_uuid() {
        for _ in 0..50 {
            let u = Uuid::new_v4();
            assert_eq!(uuid_to_pokemon(u), From::from(u));
        }
    }

    #[test]
    fn test_to_string() {
        let u = Uuid::nil();
        assert_eq!(uuid_to_pokemon(u).to_string(), "Busy bulbasaur".to_string());
    }

    #[test]
    fn test_fail_eq_str() {
        let items = &[
            ("Busy", "bulbasau"),
            ("Busy", "bulbasaua"),
            ("busy", "bulbasaur"),
        ];
        let u = Uuid::nil();
        for n in items.iter().cloned() {
            assert!(uuid_to_pokemon(u) != n.into());
            assert!(PokemonUuid::from(n) != uuid_to_pokemon(u));
        }
    }
}
