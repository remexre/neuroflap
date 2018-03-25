use std::sync::Arc;

use failure::Error;
use neuroflap_world::World;
use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder};
use vulkano::format::ClearValue;
use vulkano::framebuffer::Framebuffer;
use vulkano::image::ImageViewAccess;
use vulkano::swapchain::{acquire_next_image, AcquireError};
use vulkano::sync::GpuFuture;

use Renderer;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    position: [f32; 2],
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
    fn draw_world<I: ImageViewAccess + Send + Sync>(
        &mut self,
        world: &World,
        image: I,
    ) -> Result<AutoCommandBuffer, Error> {
        let framebuffer = Arc::new(Framebuffer::start(
            self.render_pass.clone(),
        ).add(image)?
            .build()?);

        let dynamic_state = unimplemented!();
        let vertex_buffer = unimplemented!();

        AutoCommandBufferBuilder::primary_one_time_submit(
            self.device.clone(),
            self.queue.family(),
        )?.begin_render_pass(framebuffer, false, vec![0.0, 0.0, 1.0, 1.0])?
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
