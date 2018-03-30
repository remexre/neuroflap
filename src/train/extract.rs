use std::fs::File;
use std::path::PathBuf;

use bincode::{deserialize_from, serialize_into};
use failure::Error;
use inflector::numbers::ordinalize::ordinalize;
use neuroflap_neat::Population;

/// Options taken by the `extract` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {
    /// The input generation file.
    pub generation_file: PathBuf,

    /// The index of the entry to extract.
    pub n: usize,

    /// The output genome file.
    pub genome_file: PathBuf,
}

impl Options {
    /// Runs extract mode.
    pub fn run(self) -> Result<(), Error> {
        let pop: Population = {
            let f = File::open(self.generation_file)?;
            deserialize_from(f)?
        };

        let genome = if pop.len() <= self.n {
            let n = format!("{}", self.n);
            let nth = ordinalize(&n);
            bail!("There isn't a {} species", nth)
        } else {
            &pop[self.n]
        };

        let f = File::create(self.genome_file)?;
        serialize_into(f, &genome).map_err(Error::from)
    }
}
