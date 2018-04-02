mod iter;

use std::ops::{Index, IndexMut};

use rand::Rng;

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

    /// Mutates a population.
    fn mutate<I: FnMut() -> usize, R: Rng>(&mut self, r: &mut R, mut inno: I) {
        for i in 0..self.len() {
            if r.next_f32() < self.params.mutation_rate {
                // TODO Avoid the clone.
                let mut genome = self[i].clone();
                genome.mutate(r, &mut inno, &self.params);
                self[i] = genome;
            }
        }
    }

    /// Runs a single generation. The given function evaluates an individual's
    /// fitness.
    pub fn run_generation<E, F, I, R>(
        &self,
        r: &mut R,
        mut fitness: F,
        inno: I,
    ) -> Result<Population, E>
    where
        F: FnMut(&Genome) -> Result<f32, E>,
        I: FnMut() -> usize,
        R: Rng,
    {
        let mut pop = self.clone();
        pop.mutate(r, inno);

        let fitnesses = pop.species
            .iter()
            .flat_map(|s| s)
            .map(fitness)
            .collect::<Result<Vec<f32>, E>>()?;

        println!("{:#?}", fitnesses);
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
