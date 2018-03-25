use std::time::Duration;

/// The state of the entire game world.
pub struct World {
    /// The bird's vertical velocity.
    pub velocity: f32,
}

impl World {
    /// Creates a new, empty World object.
    pub fn new() -> World {
        World { velocity: 0.0 }
    }

    /// Simulates the world, given that `dt` has passed since the last call.
    pub fn simulate(&mut self, dt: Duration) {
        // warn!("TODO World::simulate");
    }
}
