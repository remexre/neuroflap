use neuroflap_world::{World, GAP_HEIGHT, PIPE_WIDTH, PLAYER_HEIGHT,
                      PLAYER_WIDTH};
use rand::Rng;

use super::{Color::{self, *}, Vertex};

pub fn draw_world<R: Rng>(world: &World<R>) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    draw_bird(world.position, &mut vertices);
    for &(x, y) in world.pipes.iter() {
        draw_pipe(x, y, &mut vertices);
    }
    vertices
}

fn draw_bird(position: f32, vertices: &mut Vec<Vertex>) {
    const W: f32 = PLAYER_WIDTH / 2.0;
    const H: f32 = PLAYER_HEIGHT / 2.0;

    draw_rect_corners(
        (0.5 - W, position + H),
        (0.5 + W, position - H),
        BIRD,
        vertices,
    );
}

fn draw_pipe(x: f32, y: f32, vertices: &mut Vec<Vertex>) {
    const W: f32 = PIPE_WIDTH / 2.0;
    const H: f32 = GAP_HEIGHT / 2.0;

    vertices.reserve(12);
    draw_rect_corners((x - W, 1.0), (x + W, y + H), PIPE, vertices);
    draw_rect_corners((x - W, y - H), (x + W, 0.0), PIPE, vertices);
}

fn draw_rect_corners(
    ul: (f32, f32),
    br: (f32, f32),
    color: Color,
    vertices: &mut Vec<Vertex>,
) {
    vertices.reserve(6);

    vertices.push(Vertex::new(ul.0, ul.1, color));
    vertices.push(Vertex::new(ul.0, br.1, color));
    vertices.push(Vertex::new(br.0, ul.1, color));

    vertices.push(Vertex::new(br.0, br.1, color));
    vertices.push(Vertex::new(br.0, ul.1, color));
    vertices.push(Vertex::new(ul.0, br.1, color));
}

fn draw_rect_wh(
    center: (f32, f32),
    width: f32,
    height: f32,
    color: Color,
    vertices: &mut Vec<Vertex>,
) {
    draw_rect_corners(
        (center.0 - width, center.1 + height),
        (center.0 + width, center.1 - height),
        color,
        vertices,
    )
}
