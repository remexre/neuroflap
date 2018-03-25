extern crate failure;
extern crate futures;
extern crate futures_timer;
#[macro_use]
extern crate log;
extern crate neuroflap_world;
#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod builder;
mod events;

use std::sync::Arc;

use vulkano::device::{Device, Queue};
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use winit::Window;

pub use builder::Builder;
pub use events::Event;

/// The rendering subsystem.
pub struct Renderer {
    device: Arc<Device>,
    queues: Vec<Arc<Queue>>,
    instance: Arc<Instance>,
    window: Arc<Surface<Window>>,
}
