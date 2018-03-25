use std::mem::swap;

use futures::{Async, Stream};
use neuroflap_world::Event;
use winit::{Event as WinitEvent, EventsLoop};

use Renderer;

/// Events from a window.
pub struct Events {
    buffer: Vec<Event>,
    event_loop: EventsLoop,
}

impl Events {
    /// Creates a new `Events` for an `winit::EventsLoop`.
    pub(crate) fn new(event_loop: EventsLoop) -> Events {
        Events {
            buffer: Vec::new(),
            event_loop,
        }
    }
}

impl Stream for Events {
    type Item = Event;
    type Error = !;

    fn poll(&mut self) -> Result<Async<Option<Event>>, !> {
        if let Some(ev) = self.buffer.pop() {
            Ok(Async::Ready(Some(ev)))
        } else {
            let mut buffer = Vec::new();
            swap(&mut buffer, &mut self.buffer);
            self.event_loop.poll_events(|event| {
                if let Some(event) = convert_event(event) {
                    buffer.push(event);
                }
            });
            buffer.reverse();
            swap(&mut buffer, &mut self.buffer);
            debug_assert!(buffer.is_empty());

            if self.buffer.is_empty() {
                Ok(Async::NotReady)
            } else {
                debug!("Got {} events", self.buffer.len());
                self.poll()
            }
        }
    }
}

fn convert_event(event: WinitEvent) -> Option<Event> {
    use winit::{DeviceEvent, VirtualKeyCode};

    match event {
        WinitEvent::DeviceEvent {
            event: DeviceEvent::Key(key),
            ..
        } => match key.virtual_keycode {
            Some(VirtualKeyCode::Escape) => Some(Event::Quit),
            Some(VirtualKeyCode::Space) => Some(Event::Jump),
            _ => None,
        },
        _ => None,
    }
}
