use std::sync::Arc;

use failure::{err_msg, Error};
use vulkano::device::{Device, Queue};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
use vulkano::swapchain::{Surface, Swapchain};
use vulkano_win::VkSurfaceBuild;
use winit::{EventsLoop, WindowBuilder};

use events::Events;
use {Renderer, DEFAULT_HEIGHT, DEFAULT_WIDTH};

/// A builder for a Renderer and an Events.
#[derive(Clone, Default)]
pub struct Builder {
    // TODO: This will have... any sort of configuration, really... eventually.
}

impl Builder {
    /// Builds the `Renderer` and an `Events`, initializing Vulkan and creating
    /// a window.
    pub fn build(self) -> Result<(Renderer, Events), Error> {
        let instance = create_instance()?;
        debug!("Successfully created Vulkan instance.");

        let (device, queue) = {
            let physical_device = choose_physical_device(&instance)?;
            info!("Using device {:?}", physical_device.name());

            let qf = choose_queue_family(physical_device)?;
            let (device, mut queues) = build_device(physical_device, qf)?;
            ensure!(queues.len() > 0, "Device has no queues");
            (device, queues.pop().unwrap())
        };

        let event_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_dimensions(DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .with_min_dimensions(DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .with_max_dimensions(DEFAULT_WIDTH, DEFAULT_HEIGHT)
            .with_title("neuroflap")
            .build_vk_surface(&event_loop, instance.clone())?;

        let (swapchain, images) =
            make_swapchain(device.clone(), window.clone(), queue.family())?;

        let renderer = Renderer {
            device,
            images,
            instance,
            queue,
            swapchain,
            window,
        };
        let events = Events::new(event_loop);
        Ok((renderer, events))
    }
}

fn create_instance() -> Result<Arc<Instance>, Error> {
    use vulkano_win::required_extensions;

    let app_info = app_info_from_cargo_toml!();
    let extensions = required_extensions();
    Instance::new(Some(&app_info), &extensions, None).map_err(Error::from)
}

// TODO: More complex physical device selection criteria than "first detected."
fn choose_physical_device(
    instance: &Arc<Instance>,
) -> Result<PhysicalDevice, Error> {
    PhysicalDevice::enumerate(instance)
        .next()
        .ok_or_else(|| err_msg("No Vulkan devices found"))
}

// TODO: More complex criteria than "first detected."
fn choose_queue_family(
    physical_device: PhysicalDevice,
) -> Result<QueueFamily, Error> {
    physical_device
        .queue_families()
        .find(|qf| qf.supports_graphics())
        .ok_or_else(|| err_msg("Your Vulkan doesn't support graphics"))
}

fn build_device(
    physical_device: PhysicalDevice,
    queue_family: QueueFamily,
) -> Result<(Arc<Device>, Vec<Arc<Queue>>), Error> {
    use vulkano::device::DeviceExtensions;
    use vulkano::instance::Features;

    let extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let (dev, queues) = Device::new(
        physical_device,
        &Features::none(),
        &extensions,
        [(queue_family, 0.5)].iter().cloned(),
    )?;
    Ok((dev, queues.collect()))
}

fn make_swapchain<W>(
    device: Arc<Device>,
    surface: Arc<Surface<W>>,
    queue_family: QueueFamily,
) -> Result<(Arc<Swapchain<W>>, Vec<Arc<SwapchainImage<W>>>), Error> {
    use std::cmp::{max, min};

    use vulkano::image::ImageUsage;
    use vulkano::swapchain::{CompositeAlpha, PresentMode};
    use vulkano::sync::SharingMode;

    let caps = surface.capabilities(device.physical_device())?;

    let dims = caps.current_extent
        .unwrap_or([DEFAULT_WIDTH, DEFAULT_HEIGHT]);
    let buffers_count = max(
        min(2, caps.min_image_count),
        caps.max_image_count.unwrap_or(2),
    );
    let transform = caps.current_transform;
    let (format, color_space) = caps.supported_formats[0];
    let usage = ImageUsage {
        color_attachment: true,
        ..ImageUsage::none()
    };
    let sharing_mode = SharingMode::Exclusive(queue_family.id());

    Swapchain::new(
        device,
        surface,
        buffers_count,
        format,
        dims,
        1,
        usage,
        sharing_mode,
        transform,
        CompositeAlpha::Opaque,
        PresentMode::Fifo,
        true,
        None,
    ).map_err(Error::from)
}
