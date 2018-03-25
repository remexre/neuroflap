use failure::Error;

/// Options taken by the `train` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {}

impl Options {
    /// Runs for several generations.
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}
