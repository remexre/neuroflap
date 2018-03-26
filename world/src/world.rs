use std::time::Duration;

use float_ord::FloatOrd;
use rand::Rng;

/// The height of the gap between the top and bottom pipes.
pub const GAP_HEIGHT: f32 = 0.3;

const GRAVITY: f32 = -1.5;

/// The width of pipes.
pub const PIPE_WIDTH: f32 = 0.1;

/// The height of the player.
pub const PLAYER_HEIGHT: f32 = 0.05;

/// The width of the player.
pub const PLAYER_WIDTH: f32 = 0.05;

/// The speed of the pipes.
pub const SPEED: f32 = 0.25;

/// The state of the entire game world.
pub struct World<R: Rng> {
    /// The currently existing pipes.
    pub pipes: Vec<(f32, f32)>,

    /// The bird's vertical position.
    pub position: f32,

    /// The total time survived.
    pub survived: f32,

    /// The bird's vertical velocity.
    pub velocity: f32,

    /// The random number generator for pipe positions.
    rng: R,
}

impl<R: Rng> World<R> {
    /// Creates a new, empty World object.
    pub fn new(rng: R) -> World<R> {
        World {
            pipes: Vec::new(),
            position: 0.5,
            survived: 0.0,
            velocity: 0.0,
            rng,
        }
    }

    /// Checks if the player intersects a pipe or the ground.
    pub fn intersects(&self) -> bool {
        self.position == 0.0 || unimplemented!()
    }

    /// Simulates the world, given that `dt` has passed since the last call.
    pub fn simulate(&mut self, dt: Duration) {
        let dt = {
            let dt_secs = dt.as_secs() as f32;
            let dt_nanos = dt.subsec_nanos() as f32 / 1e9;
            dt_secs + dt_nanos
        };

        self.velocity = clamp(-2.0, 2.0, self.velocity + dt * GRAVITY);
        self.position = clamp(0.0, 1.0, self.position + dt * self.velocity);

        for i in 0..self.pipes.len() {
            self.pipes[i].0 -= SPEED * dt;
        }
        self.pipes
            .retain(|&(x, _)| x > -PIPE_WIDTH / 2.0);
        if self.pipes
            .iter()
            .cloned()
            .map(|(x, _)| FloatOrd(x))
            .max()
            .map(|x| x.0)
            .unwrap_or(0.0) < 0.5
        {
            let y = self.rng.gen_range(GAP_HEIGHT, 1.0 - GAP_HEIGHT);
            self.pipes.push((1.0, y));
        }

        self.survived += dt;
    }
}

fn clamp<T: PartialOrd>(min: T, max: T, x: T) -> T {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
