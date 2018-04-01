use genome::Genome;
use species::Species;

/// An iterator over genomes in a species.
pub struct SpeciesIter<'a> {
    pub(crate) n: usize,
    pub(crate) species: &'a Species,
}

impl<'a> Iterator for SpeciesIter<'a> {
    type Item = &'a Genome;

    fn next(&mut self) -> Option<&'a Genome> {
        let n = self.n;
        if n >= self.species.len() {
            None
        } else {
            self.n += 1;
            Some(&self.species[n])
        }
    }
}
