use failure::Error;
use neuroflap_world::{Event, World};

/// Options taken by the `play` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {}

impl Options {
    /// Starts the game in play mode.
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}
