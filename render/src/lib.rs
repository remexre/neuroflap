//! The renderer for neuroflap.
#![feature(conservative_impl_trait)]
#![warn(missing_docs)]

#[macro_use]
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate neuroflap_world;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

mod builder;
mod events;
mod render;
mod shaders;

use std::sync::Arc;

use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{RenderPass, RenderPassDesc};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::swapchain::{Surface, Swapchain};
use winit::Window;

pub use builder::Builder;
use render::Vertex;

const DEFAULT_WIDTH: u32 = 640;
const DEFAULT_HEIGHT: u32 = 480;

/// The rendering subsystem.
pub struct Renderer {
    device: Arc<Device>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    pipeline: GraphicsPipeline<
        SingleBufferDefinition<Vertex>,
        Box<PipelineLayoutAbstract + Sync + Send>,
        Arc<RenderPass<impl RenderPassDesc>>,
    >,
    queue: Arc<Queue>,
    recreate_swapchain: bool,
    render_pass: Arc<RenderPass<impl RenderPassDesc>>,
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,
}
