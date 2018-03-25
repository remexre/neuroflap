use failure::Error;
use neuroflap_world::World;
use vulkano::swapchain::acquire_next_image;
use vulkano::sync::GpuFuture;

use Renderer;

impl Renderer {
    /// Renders the world into a frame and swaps buffers.
    pub fn render_world(&mut self, world: &World) -> Result<(), Error> {
        let (index, future) = acquire_next_image(self.swapchain.clone(), None)?;
        warn!("TODO Renderer::render_world");
        future
            .then_swapchain_present(
                self.queue.clone(),
                self.swapchain.clone(),
                index,
            )
            .then_signal_fence()
            .flush()
            .map_err(Error::from)
    }
}
