//! The world type, as well as other "essential" types for neuroflap.
#![warn(missing_docs)]

#[macro_use]
extern crate log;

mod controller;
mod event;
mod world;

pub use controller::Controller;
pub use event::Event;
pub use world::World;
