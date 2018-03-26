//! The world type, as well as other "essential" types for neuroflap.
#![warn(missing_docs)]

extern crate failure;
extern crate float_ord;
extern crate futures;
#[macro_use]
extern crate log;
extern crate rand;

mod bounding_box;
mod event;
mod util;
mod world;

use std::time::Instant;

use failure::{Error, Fail};
use futures::Stream;
use util::take_ready;

pub use event::Event;
pub use world::{World, GAP_HEIGHT, PIPE_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH};

/// Runs a single instance of the game, with the given controller, renderer,
/// and RNG.
pub fn run_one<Controller, Renderer, Rng, E>(
    mut controller: Controller,
    mut renderer: Renderer,
    rng: Rng,
) -> Result<Option<f32>, Error>
where
    Controller: Stream<Item = Event, Error = E>,
    Renderer: FnMut(&World<Rng>) -> Result<(), Error>,
    Rng: ::rand::Rng,
    E: Fail,
{
    let mut world = World::new(rng);
    let mut timer = Instant::now();
    loop {
        // Handle events.
        let mut quit = false;
        take_ready(&mut controller, |event| {
            debug!("Got event {:?}", event);
            match event {
                Event::Jump => world.velocity = 0.75,
                Event::Quit => quit = true,
            }
        })?;
        if quit {
            return Ok(None);
        }

        // Update physics.
        let dt = timer.elapsed();
        timer = Instant::now();
        world.simulate(dt);

        // Check for lossage.
        if world.player_intersects_object() {
            return Ok(Some(world.survived));
        }

        // Render the new frame.
        renderer(&world)?;
    }
}
