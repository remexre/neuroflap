extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
#[cfg(feature = "neuroflap-control-player")]
extern crate neuroflap_control_player;
#[cfg(feature = "neuroflap-render")]
extern crate neuroflap_render;
extern crate neuroflap_world;
extern crate rand;
extern crate stderrlog;
#[macro_use]
extern crate structopt;

#[cfg(feature = "play")]
mod play;
#[cfg(feature = "simulate")]
mod simulate;
#[cfg(feature = "train")]
mod train;
mod util;

use std::process::exit;

use structopt::StructOpt;

fn main() {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose + 2)
        .init()
        .expect("Failed to start logger");

    let result = match options.subcommand {
        #[cfg(feature = "train")]
        Subcommand::Extract(extract) => extract.run(),

        #[cfg(feature = "train")]
        Subcommand::New(new) => new.run(),

        #[cfg(feature = "play")]
        Subcommand::Play(play) => play.run(),

        #[cfg(feature = "simulate")]
        Subcommand::Simulate(simulate) => simulate.run(),

        #[cfg(feature = "train")]
        Subcommand::Train(train) => train.run(),
    };

    if let Err(err) = result {
        error!("{:#?}", err);
        error!("{}", err);
        exit(1);
    }
}

#[derive(Debug, StructOpt)]
struct Options {
    /// The controller.
    #[structopt(subcommand)]
    subcommand: Subcommand,

    /// Silence all log output.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Increase log verbosity (-v, -vv, -vvv, etc).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

#[derive(Debug, StructOpt)]
enum Subcommand {
    /// Extracts a single neural net from a generation file.
    #[cfg(feature = "train")]
    #[structopt(name = "extract")]
    Extract(train::ExtractOptions),

    /// Creates a new generation file.
    #[cfg(feature = "train")]
    #[structopt(name = "new")]
    New(train::NewOptions),

    /// Starts the game as a human-played game.
    #[cfg(feature = "play")]
    #[structopt(name = "play")]
    Play(play::Options),

    /// Starts the game as a simulator for a single already-trained neural net.
    #[cfg(feature = "simulate")]
    #[structopt(name = "simulate")]
    Simulate(simulate::Options),

    /// Trains for a certain number of generations.
    #[cfg(feature = "train")]
    #[structopt(name = "train")]
    Train(train::TrainOptions),
}
