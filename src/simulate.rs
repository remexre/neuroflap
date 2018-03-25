use failure::Error;
use neuroflap_world::{Event, World};

/// Options taken by the `simulate` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {}

impl Options {
    /// Starts the game in simulation mode.
    pub fn run(self) -> Result<(), Error> {
        unimplemented!()
    }
}
