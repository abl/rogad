use crate::amixer::AMIXER;
use crate::gpio::GPIO;
use crate::input;
use evdev_rs::enums::{EventCode, EV_SW};
use std::thread;

pub struct HeadphoneMonitor {}
impl HeadphoneMonitor {
    pub fn start() {
        HeadphoneMonitor::initialize_headphone_state();
        thread::spawn(HeadphoneMonitor::read_headphone_events);
    }

    fn initialize_headphone_state() {
        let reader = GPIO::new(86);
        let state = reader.read_bool();

        if state {
            // headphones inserted
            AMIXER.set_headphone();
        } else {
            // headphones not inserted
            AMIXER.set_speaker();
        }
    }

    fn read_headphone_events() {
        let mut event_stream = input::InputEventStream::new("platform-rk817-sound-event", false);
        loop {
            let event = event_stream.next();
            if event.event_code != EventCode::EV_SW(EV_SW::SW_HEADPHONE_INSERT) {
                continue;
            }
            printlog!("platform-rk817-sound-event {:#?}", event);
            if event.value == 0 {
                // headphones inserted
                AMIXER.set_headphone();
            } else {
                // event.value == 1
                // headphones removed
                AMIXER.set_speaker();
            }
        }
    }
}
