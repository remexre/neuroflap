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

    /// The number of members in the population.
    pub population_size: usize,
}
