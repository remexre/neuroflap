mod draw;

use std::sync::Arc;

use failure::Error;
use neuroflap_world::World;
use rand::Rng;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::submit::SubmitPresentError;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::format::ClearValue;
use vulkano::framebuffer::Framebuffer;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{acquire_next_image, AcquireError};
use vulkano::sync::{FlushError, GpuFuture};

use Renderer;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    BIRD = 0,
    PIPE = 1,
}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    color: usize,
    position: [f32; 2],
}

impl Vertex {
    /// Creates a new Vertex.
    pub fn new(x: f32, y: f32, color: Color) -> Vertex {
        Vertex {
            color: color as usize,
            position: [x, y],
        }
    }
}

impl_vertex!(Vertex, position);

impl Renderer {
    /// Renders the world into a frame and swaps buffers.
    pub fn render_world<R: Rng>(
        &mut self,
        world: &World<R>,
    ) -> Result<(), Error> {
        if self.recreate_swapchain {
            let (x, y) = self.surface.window().get_inner_size().unwrap();
            let (swapchain, images) =
                self.swapchain.recreate_with_dimension([x, y])?;

            self.swapchain = swapchain;
            self.images = images;
            self.recreate_swapchain = false;
        }

        let recreate = self.render_world_inner(world)
            .map(|()| false)
            .or_else(|err| match err.downcast() {
                Ok(AcquireError::OutOfDate) => Ok(true),
                Ok(err) => Err(err.into()),
                Err(err) => Err(err),
            })
            .or_else(|err| match err.downcast() {
                Ok(FlushError::OutOfDate) => Ok(true),
                Ok(err) => Err(err.into()),
                Err(err) => Err(err),
            })
            .or_else(|err| match err.downcast() {
                Ok(SubmitPresentError::OutOfDate) => Ok(true),
                Ok(err) => Err(err.into()),
                Err(err) => Err(err),
            })?;

        self.recreate_swapchain = recreate;
        Ok(())
    }

    fn render_world_inner<R: Rng>(
        &mut self,
        world: &World<R>,
    ) -> Result<(), Error> {
        let (index, future) = acquire_next_image(self.swapchain.clone(), None)?;

        let image = self.images[index].clone();

        let framebuffer = Arc::new(Framebuffer::start(
            self.render_pass.clone(),
        ).add(image)?
            .build()?);

        let dims = self.swapchain.dimensions();
        let dynamic_state = DynamicState {
            viewports: Some(vec![
                Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [dims[0] as f32, dims[1] as f32],
                    depth_range: 0.0..1.0,
                },
            ]),
            ..DynamicState::none()
        };

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            draw::draw_world(world).into_iter(),
        ).unwrap();

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )?.begin_render_pass(
            framebuffer,
            false,
            vec![ClearValue::Float([0.5, 0.5, 1.0, 1.0])],
        )?
            .draw(
                self.pipeline.clone(),
                dynamic_state,
                vertex_buffer.clone(),
                (),
                (),
            )?
            .end_render_pass()?
            .build()?;

        future
            .then_execute(self.queue.clone(), command_buffer)?
            .then_swapchain_present(
                self.queue.clone(),
                self.swapchain.clone(),
                index,
            )
            .then_signal_fence_and_flush()?
            .cleanup_finished();

        Ok(())
    }
}
