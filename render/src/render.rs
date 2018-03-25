use std::sync::Arc;

use failure::Error;
use neuroflap_world::World;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder,
                              DynamicState};
use vulkano::format::ClearValue;
use vulkano::framebuffer::Framebuffer;
use vulkano::image::ImageViewAccess;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{acquire_next_image, AcquireError};
use vulkano::sync::GpuFuture;

use Renderer;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    /// Creates a new Vertex.
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex {
            position: [x, y],
        }
    }
}

impl_vertex!(Vertex, position);

impl Renderer {
    /// Renders the world into a frame and swaps buffers.
    pub fn render_world(&mut self, world: &World) -> Result<(), Error> {
        if self.recreate_swapchain {
            let (x, y) = self.surface.window().get_inner_size().unwrap();
            let (swapchain, images) =
                self.swapchain.recreate_with_dimension([x, y])?;

            self.swapchain = swapchain;
            self.images = images;
            self.recreate_swapchain = false;
        }

        let (index, future) =
            match acquire_next_image(self.swapchain.clone(), None) {
                Ok(x) => x,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return Ok(());
                }
                Err(err) => return Err(err.into()),
            };

        let image = self.images[index].clone();
        let command_buffer = self.draw_world(world, image)?;

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

    /// Builds a buffer of draw commands for rendering the world.
    fn draw_world<I: ImageViewAccess + Send + Sync + 'static>(
        &mut self,
        world: &World,
        image: I,
    ) -> Result<AutoCommandBuffer, Error> {
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
            vec![
                Vertex::new(-0.5, -0.25),
                Vertex::new(0.0, 0.5),
                Vertex::new(0.25, -0.1),
            ].into_iter(),
        ).unwrap();

        AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )?.begin_render_pass(
            framebuffer,
            false,
            vec![ClearValue::Float([0.0, 0.0, 1.0, 1.0])],
        )?
            .draw(
                self.pipeline.clone(),
                dynamic_state,
                vertex_buffer.clone(),
                (),
                (),
            )?
            .end_render_pass()?
            .build()
            .map_err(Error::from)
    }
}
