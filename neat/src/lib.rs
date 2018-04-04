//! An implementation of NeuroEvolution of Augmenting Topologies, as described
//! in Evolving Neural Networks through Augmenting Topologies by Stanley and
//! Miikkulainen.

#[macro_use]
extern crate derive_more;
extern crate float_ord;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate structopt;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod activation;
mod crossover;
mod genome;
mod network;
mod params;
mod population;
mod species;
mod util;

pub use activation::Activation;
pub use genome::{Gene, Genome};
pub use network::Network;
pub use params::Params;
pub use population::{Population, PopulationIter};
