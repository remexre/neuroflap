use std::fs::File;
use std::path::PathBuf;

use bincode::{deserialize_from, serialize_into};
use failure::Error;

/// Options taken by the `train` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {
    /// The generation being trained.
    pub generation_file: PathBuf,

    /// A directory where generation snapshots are stored.
    #[structopt(long = "results")]
    pub results_dir: Option<PathBuf>,
}

impl Options {
    /// Runs for several generations.
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}
