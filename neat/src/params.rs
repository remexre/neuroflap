use std::fmt::{Display, Formatter, Result as FmtResult};

use activation::Activation;

/// The parameters used to train a population.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, StructOpt)]
pub struct Params {
    /// The weight for excess genes in the difference function.
    #[structopt(default_value = "1.0", long = "c1")]
    pub c1: f32,

    /// The weight for disjoint genes in the difference function.
    #[structopt(default_value = "1.0", long = "c2")]
    pub c2: f32,

    /// The weight for the mean weight difference between matching genes in the
    /// difference function.
    #[structopt(default_value = "0.4", long = "c3")]
    pub c3: f32,

    /// The activation function to use. Valid values are: ReLU, Sigmoid, Tanh.
    #[serde(default, with = "::util::tofromstr")]
    #[structopt(default_value = "Sigmoid", long = "activation", short = "a")]
    pub activation: Activation,

    /// The acceptable value for the difference function before two individuals
    /// are counted as being from a different species.
    #[structopt(default_value = "3.0", long = "delta-cutoff", short = "d")]
    pub delta_cutoff: f32,

    /// The chance an individual is mutated.
    #[structopt(default_value = "0.5", long = "mutation-rate", short = "m")]
    pub mutation_rate: f32,

    /// The maximum magnitude by which a reweight mutation affects a gene.
    #[structopt(default_value = "0.5", long = "reweight-amount")]
    pub reweight_amount: f32,

    /// The chance an individual gene is mutated during a reweight mutation.
    #[structopt(default_value = "0.5", long = "reweight-rate")]
    pub reweight_rate: f32,

    /// The number of members in the population.
    #[structopt(default_value = "300", long = "population", short = "p")]
    pub population_size: usize,
}

impl Display for Params {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "c1 = {}, c2 = {}, c3 = {}, activation = {}, delta_cutoff = {}, population_size = {}",
            self.c1, self.c2, self.c3, self.activation, self.delta_cutoff, self.population_size,
        )
    }
}
