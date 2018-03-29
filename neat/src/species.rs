use genome::Genome;

/// A single species. Just a newtype.
#[derive(Clone, Debug, Deserialize, From, Index, Into, Serialize)]
pub struct Species(pub Vec<Genome>);

impl Species {
    /// Creates a new, empty species.
    pub fn empty() -> Species {
        Species(Vec::new())
    }

    /// Creates a new species with a default genome and the given size.
    pub fn with_size(population_size: usize) -> Species {
        Species(vec![Genome::new(); population_size])
    }
}
