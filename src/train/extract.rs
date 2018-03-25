use failure::Error;

/// Options taken by the `extract` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {}

impl Options {
    /// Runs extract mode.
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}
