mod iter;

use std::ops::{Index, IndexMut};

use activation::Activation;
use genome::Genome;
use params::Params;
use species::Species;

pub use self::iter::PopulationIter;

/// A collection of organisms that can be evaluated and trained as one.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Population {
    /// The parameters used for the next generation.
    pub params: Params,

    generation: usize,
    species: Vec<Species>,
}

impl Population {
    /// Creates a new population with the given parameters.
    pub fn new(params: Params) -> Population {
        Population {
            generation: 0,
            params,
            species: vec![Species::with_size(params.population_size)],
        }
    }

    /// Returns the generation number this population is at.
    pub fn generation(&self) -> usize {
        self.generation
    }

    /// Returns the number of individuals in the population.
    pub fn len(&self) -> usize {
        self.species.iter().map(|s| s.len()).sum()
    }

    /// Runs a single generation. The given function evaluates an individual's
    /// fitness.
    pub fn run_generation<E, F>(&self, mut fitness: F) -> Result<Population, E>
    where
        F: FnMut(&Genome) -> Result<f32, E>,
    {
        unimplemented!()
    }
}

impl Index<usize> for Population {
    type Output = Genome;

    fn index(&self, n: usize) -> &Genome {
        let mut n = n;
        for s in self.species.iter() {
            if n >= s.len() {
                n -= s.len();
                continue;
            } else {
                return &s[n];
            }
        }
        panic!("No genome {} in population", n);
    }
}

impl IndexMut<usize> for Population {
    fn index_mut(&mut self, n: usize) -> &mut Genome {
        let mut n = n;
        for s in self.species.iter_mut() {
            if n >= s.len() {
                n -= s.len();
                continue;
            } else {
                return &mut s[n];
            }
        }
        panic!("No genome {} in population", n);
    }
}

impl<'a> IntoIterator for &'a Population {
    type IntoIter = PopulationIter<'a>;
    type Item = &'a Genome;

    fn into_iter(self) -> PopulationIter<'a> {
        PopulationIter {
            n: 0,
            pop: self,
        }
    }
}
