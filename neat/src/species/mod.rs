mod iter;

use genome::Genome;

pub use self::iter::SpeciesIter;

/// A single species. Just a newtype.
#[derive(Clone, Debug, Deserialize, From, Index, IndexMut, Into, Serialize)]
pub struct Species(pub Vec<Genome>);

impl Species {
    /// Creates a new, empty species.
    pub fn empty() -> Species {
        Species(Vec::new())
    }

    /// Returns the number of individuals in the species.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Creates a new species with a default genome and the given size.
    pub fn with_size(population_size: usize) -> Species {
        Species(vec![Genome::new(); population_size])
    }
}

impl<'a> IntoIterator for &'a Species {
    type IntoIter = SpeciesIter<'a>;
    type Item = &'a Genome;

    fn into_iter(self) -> SpeciesIter<'a> {
        SpeciesIter {
            n: 0,
            species: self,
        }
    }
}
