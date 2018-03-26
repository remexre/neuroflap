//! The world type, as well as other "essential" types for neuroflap.
#![warn(missing_docs)]

extern crate float_ord;
#[macro_use]
extern crate log;
extern crate rand;

mod controller;
mod event;
mod world;

pub use controller::Controller;
pub use event::Event;
pub use world::{World, GAP_HEIGHT, PIPE_WIDTH, PLAYER_HEIGHT, PLAYER_WIDTH};
