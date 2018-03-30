use std::fs::File;
use std::path::PathBuf;

use bincode::serialize_into;
use failure::Error;
use neuroflap_neat::{Params, Population};
use structopt::StructOpt;
use structopt::clap::{App, Arg, ArgMatches};

/// Options taken by the `new` subcommand.
#[derive(Clone, Debug)]
pub struct Options {
    /// The generation file to write.
    pub generation_file: PathBuf,

    /// The training parameters to put in the file.
    pub params: Params,
}

#[doc(hidden)]
impl Options {
    pub fn augment_clap<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b> {
        Params::augment_clap(app).arg(
            Arg::with_name("generation_file")
                .takes_value(true)
                .multiple(false)
                .required(true)
                .help("The generation file to write."),
        )
    }

    pub fn is_subcommand() -> bool {
        Params::is_subcommand()
    }
}

impl StructOpt for Options {
    fn clap<'a, 'b>() -> App<'a, 'b> {
        Params::clap()
    }

    fn from_clap(matches: &ArgMatches) -> Options {
        let generation_file = matches
            .value_of_os("generation_file")
            .map(PathBuf::from)
            .unwrap();
        Options {
            generation_file,
            params: Params::from_clap(matches),
        }
    }
}

impl Options {
    /// Creates a new generation file.
    pub fn run(self) -> Result<(), Error> {
        let pop = Population::new(self.params);

        let f = File::create(self.generation_file)?;
        serialize_into(f, &pop).map_err(Error::from)
    }
}
