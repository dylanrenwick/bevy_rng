use bevy::prelude::*;
use rand::SeedableRng;
use rand_seeder::Seeder;
use rand_xorshift::XorShiftRng;
use std::ops::{Deref, DerefMut};

pub use rand::Rng as _;

/// `RngPlugin` allows you to inject a (optionally seeded) random number
/// generator into your systems.
///
/// Once the plugin is active, you can use `Local<bevy_rng::Rng>` to get the
/// rng instance.
///
/// Using `Local<T>` ensures you get a unique copy of the rng for each
/// individual system, which is important, because systems have no deterministic
/// order, which will result in non-deterministic rng results.
///
/// You are still responsible for deterministically generating random numbers
/// _inside_ an individual system, which (currently) means you can't generate
/// random numbers when iterating over entities, as entity iteration also isn't
/// ordered currently.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RngPlugin {
    seed: Option<Seed>,
}

impl From<String> for RngPlugin {
    fn from(seed: String) -> Self {
        Self {
            seed: Some(Seed::String(seed)),
        }
    }
}

impl From<&str> for RngPlugin {
    fn from(seed: &str) -> Self {
        Self {
            seed: Some(Seed::String(seed.to_owned())),
        }
    }
}

impl From<u64> for RngPlugin {
    fn from(seed: u64) -> Self {
        Self {
            seed: Some(Seed::Number(seed)),
        }
    }
}

impl Plugin for RngPlugin {
    fn build(&self, app: &mut AppBuilder) {
        if let Some(seed) = &self.seed {
            app.add_resource(seed.clone());
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Seed {
    Number(u64),
    String(String),
}

/// The random number generator.
///
/// This wraps `rand`'s `XorShiftRng` random number generator.
///
/// See the `rand::Rng` trait for more details on how to generate random data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng {
    inner: XorShiftRng,
}

impl Deref for Rng {
    type Target = XorShiftRng;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Rng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FromResources for Rng {
    fn from_resources(resources: &Resources) -> Self {
        let inner = match resources.get::<Seed>() {
            Some(seed) => match seed.deref() {
                Seed::String(seed) => Seeder::from(seed.as_str()).make_rng(),
                Seed::Number(num) => XorShiftRng::seed_from_u64(*num),
            },
            None => XorShiftRng::seed_from_u64(0),
        };

        Self { inner }
    }
}
