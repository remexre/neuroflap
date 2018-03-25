use failure::Error;

/// Options taken by the `new` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {}

impl Options {
    /// Creates a new generation file.
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}
