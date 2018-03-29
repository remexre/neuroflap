use std::path::PathBuf;

use failure::Error;

/// Options taken by the `train` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {
    /// A directory where generation snapshots are stored.
    pub results_dir: Option<PathBuf>,
}

impl Options {
    /// Runs for several generations.
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}
