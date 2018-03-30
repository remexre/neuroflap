use genome::Genome;
use population::Population;

/// An iterator over genomes in a population.
pub struct PopulationIter<'a> {
    pub(crate) n: usize,
    pub(crate) pop: &'a Population,
}

impl<'a> Iterator for PopulationIter<'a> {
    type Item = &'a Genome;

    fn next(&mut self) -> Option<&'a Genome> {
        let n = self.n;
        if n >= self.pop.len() {
            None
        } else {
            self.n += 1;
            Some(&self.pop[n])
        }
    }
}
