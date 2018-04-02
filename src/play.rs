use std::time::Instant;

use failure::Error;
use neuroflap_render::Builder;
use neuroflap_world::run_one;
use rand::StdRng;

/// Options taken by the `play` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {}

impl Options {
    /// Starts the game in play mode.
    pub fn run(self) -> Result<(), Error> {
        let mut rng = StdRng::new()?;
        let (mut renderer, mut events) = Builder::default().build()?;
        let mut timer = Instant::now();
        loop {
            let r = run_one(
                &mut events,
                |w| renderer.render_world(w),
                &mut rng,
                || {
                    let d = timer.elapsed();
                    timer = Instant::now();
                    d
                },
            );

            match r {
                Ok(Some(time)) => info!("Got {} sec", time),
                Ok(None) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }
}
