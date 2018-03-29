use activation::Activation;
use genome::Genome;
use params::Params;
use species::Species;

/// A collection of organisms that can be evaluated and trained as one.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Population {
    /// The parameters used for the next generation.
    pub params: Params,

    species: Vec<Species>,
}

impl Population {
    /// Creates a new population with the given parameters.
    pub fn new(params: Params) -> Population {
        Population {
            params,
            species: vec![Species::with_size(params.population_size)],
        }
    }

    /// Runs a single generation. The given function evaluates an individual's
    /// fitness.
    pub fn run_generation<E, F>(&self, fitness: F) -> Result<Population, E>
    where
        F: Fn(&Genome) -> Result<f32, E>,
    {
        unimplemented!()
    }
}
