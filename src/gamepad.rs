use crate::input::InputEventStream;
use evdev_rs::InputEvent;
use std::fs::File;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct GamepadMonitor {
    sender: Sender<InputEvent>,
    event_file: File,
}
impl GamepadMonitor {
    pub fn start(event_file: File) -> Receiver<InputEvent> {
        let (sender, receiver) = channel();
        let mut monitor = GamepadMonitor { sender, event_file };
        thread::spawn(move || monitor.read_gamepad_events());
        receiver
    }

    fn read_gamepad_events(&mut self) {
        let mut event_stream = InputEventStream::clone(self.event_file.try_clone().ok().unwrap());
        loop {
            let event = event_stream.next();
            self.sender.send(event).unwrap();
        }
    }
}
