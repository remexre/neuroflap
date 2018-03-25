use std::time::Instant;

use failure::Error;
use neuroflap_render::Builder;
use neuroflap_world::{Event, World};

use util::take_ready;

/// Options taken by the `play` subcommand.
#[derive(Debug, StructOpt)]
pub struct Options {}

impl Options {
    /// Starts the game in play mode.
    pub fn run(self) -> Result<(), Error> {
        let mut world = World::new();
        let (mut renderer, mut events) = Builder::default().build()?;

        let mut keep_going = true;
        let mut timer = Instant::now();
        while keep_going {
            // Handle events.
            take_ready(&mut events, |event| match event {
                Event::Quit => keep_going = false,
                _ => unimplemented!("{:#?}", event),
            })?;

            // Update physics.
            let dt = timer.elapsed();
            timer = Instant::now();
            world.simulate(dt);

            // Render the new frame.
            renderer.render_world(&world)?;
        }

        info!("Quitting...");
        Ok(())
    }
}
