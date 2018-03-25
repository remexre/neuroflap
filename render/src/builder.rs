use std::sync::Arc;

use failure::{err_msg, Error};
use vulkano::device::{Device, Queue};
use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
use vulkano_win::VkSurfaceBuild;
use winit::{EventsLoop, WindowBuilder};

use Renderer;
use events::Events;

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

        let (device, queues) = {
            let physical_device = choose_physical_device(&instance)?;
            info!("Using device {:?}", physical_device.name());

            let qf = choose_queue_family(physical_device)?;
            build_device(physical_device, qf)?
        };

        let event_loop = EventsLoop::new();

        let window = WindowBuilder::new()
            .with_dimensions(640, 480)
            .with_min_dimensions(640, 480)
            .with_max_dimensions(640, 480)
            .with_title("neuroflap")
            .build_vk_surface(&event_loop, instance.clone())?;

        let renderer = Renderer {
            device,
            instance,
            queues,
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

    let (dev, queues) = Device::new(
        physical_device,
        &Features::none(),
        &DeviceExtensions::none(),
        [(queue_family, 0.5)].iter().cloned(),
    )?;
    Ok((dev, queues.collect()))
}
