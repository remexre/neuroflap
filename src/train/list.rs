use std::fs::File;
use std::path::PathBuf;

use bincode::deserialize_from;
use failure::Error;
use neuroflap_neat::Population;

/// Options taken by the `list` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {
    /// The generation file.
    pub generation_file: PathBuf,
}

impl Options {
    /// Runs list mode.
    pub fn run(self) -> Result<(), Error> {
        let pop: Population = {
            let f = File::open(self.generation_file)?;
            deserialize_from(f)?
        };

        println!("params: {}", pop.params);
        for (n, genome) in pop.into_iter().enumerate() {
            println!("{}: {}", n, genome);
        }

        Ok(())
    }
}
