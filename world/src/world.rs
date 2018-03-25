use std::time::Duration;

/// The state of the entire game world.
pub struct World {}

impl World {
    /// Creates a new, empty World object.
    pub fn new() -> World {
        World {}
    }

    /// Simulates the world, given that `dt` has passed since the last call.
    pub fn simulate(&mut self, dt: Duration) {
        warn!("TODO World::simulate");
    }
}
