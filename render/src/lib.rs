#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate neuroflap_world;
#[macro_use]
extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod builder;
mod events;
mod render;

use std::sync::Arc;

use vulkano::device::{Device, Queue};
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::swapchain::{Surface, Swapchain};
use winit::Window;

pub use builder::Builder;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 480;

/// The rendering subsystem.
pub struct Renderer {
    device: Arc<Device>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    instance: Arc<Instance>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    window: Arc<Surface<Window>>,
}
