use neuroflap_world::{World, GAP_HEIGHT, PIPE_WIDTH, PLAYER_HEIGHT,
                      PLAYER_WIDTH};
use rand::Rng;

use super::Color::*;
use super::Vertex;

pub fn draw_world<R: Rng>(world: &World<R>) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    draw_bird(world.position, &mut vertices);
    for &(x, y) in world.pipes.iter() {
        draw_pipe(x, y, &mut vertices);
    }
    vertices
}

fn draw_bird(position: f32, vertices: &mut Vec<Vertex>) {
    vertices.reserve(6);

    const W: f32 = PLAYER_WIDTH / 2.0;
    const H: f32 = PLAYER_HEIGHT / 2.0;

    vertices.push(Vertex::new(0.5 - W, position - H, BIRD));
    vertices.push(Vertex::new(0.5 + W, position + H, BIRD));
    vertices.push(Vertex::new(0.5 - W, position + H, BIRD));

    vertices.push(Vertex::new(0.5 + W, position + H, BIRD));
    vertices.push(Vertex::new(0.5 - W, position - H, BIRD));
    vertices.push(Vertex::new(0.5 + W, position - H, BIRD));
}

fn draw_pipe(x: f32, y: f32, vertices: &mut Vec<Vertex>) {
    vertices.reserve(12);

    const W: f32 = PIPE_WIDTH / 2.0;
    const H: f32 = GAP_HEIGHT / 2.0;

    vertices.push(Vertex::new(x - W, y + H, PIPE));
    vertices.push(Vertex::new(x + W, 1.0, PIPE));
    vertices.push(Vertex::new(x - W, 1.0, PIPE));

    vertices.push(Vertex::new(x + W, 1.0, PIPE));
    vertices.push(Vertex::new(x - W, y + H, PIPE));
    vertices.push(Vertex::new(x + W, y + H, PIPE));

    vertices.push(Vertex::new(x - W, 0.0, PIPE));
    vertices.push(Vertex::new(x + W, y - H, PIPE));
    vertices.push(Vertex::new(x - W, y - H, PIPE));

    vertices.push(Vertex::new(x + W, y - H, PIPE));
    vertices.push(Vertex::new(x - W, 0.0, PIPE));
    vertices.push(Vertex::new(x + W, 0.0, PIPE));
}
