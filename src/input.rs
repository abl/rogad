use evdev_rs::{Device, GrabMode, InputEvent, UInputDevice};
use std::fs::File;

pub struct InputEventStream {
    device: Device,
    new_device: Option<UInputDevice>,
    pub event_file: File,
}
impl InputEventStream {
    pub fn clone(event_file: File) -> InputEventStream {
        let mut device = Device::new().unwrap();
        device.set_fd(event_file.try_clone().ok().unwrap()).unwrap();
        InputEventStream {
            device,
            new_device: None,
            event_file,
        }
    }

    pub fn new(source_name: &str, blocking: bool) -> InputEventStream {
        let path = format!("/dev/input/by-path/{}", source_name);
        let event_file;
        event_file = File::open(path).expect("Unable to open input source");
        let mut device = Device::new().unwrap();
        device.set_fd(event_file.try_clone().ok().unwrap()).unwrap();

        if blocking {
            device.grab(GrabMode::Grab).ok();
            let new_device = UInputDevice::create_from_device(&device).unwrap();

            return InputEventStream {
                device,
                new_device: Some(new_device),
                event_file,
            };
        }
        InputEventStream {
            device,
            new_device: None,
            event_file,
        }
    }

    pub fn next(&mut self) -> InputEvent {
        let event = self
            .device
            .next_event(evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING);
        let event = event.ok().unwrap();
        event.1
    }

    pub fn send(&mut self, event: InputEvent) {
        self.new_device
            .as_ref()
            .expect("Tried to send to read-only device")
            .write_event(&event)
            .ok();
    }
}
