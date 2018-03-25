extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate neuroflap_control_player;
extern crate neuroflap_render;
extern crate neuroflap_world;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate tokio;

mod options;

use std::process::exit;

use failure::Error;
use neuroflap_render::{Builder, Event};
use neuroflap_world::World;
use structopt::StructOpt;

use options::Options;

fn main() {
    let options = Options::from_args();
    stderrlog::new()
        .quiet(options.quiet)
        .verbosity(options.verbose + 2)
        .init()
        .expect("Failed to start logger");

    if let Err(err) = run(options) {
        error!("{}", err);
        exit(1);
    }
}

fn run(_options: Options) -> Result<(), Error> {
    let mut _world = World::new();
    let (renderer, events) = Builder::default().build()?;

    //tokio::run(game(world, renderer, events));
    Ok(())
}
