#[macro_use]
mod printlog;
mod amixer;
mod brightness;
mod gamepad;
mod gpio;
mod headphone;
mod input;

use evdev_rs::enums::{EventCode, EV_KEY};
use gamepad::GamepadMonitor;
use headphone::HeadphoneMonitor;
use input::InputEventStream;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::{thread, time};

fn process_events(key_states: &mut HashMap<EV_KEY, Instant>) {
    let now = Instant::now();
    for (key, val) in key_states.iter_mut() {
        if val.checked_duration_since(now) == None {
            *val = now + Duration::from_millis(200);
            printlog!("{:#?}", key);
            process_command(key);
        }
    }
}

fn process_command(key: &EV_KEY) {
    match key {
        EV_KEY::BTN_DPAD_LEFT => {
            decrease_brightness();
        }
        EV_KEY::BTN_DPAD_RIGHT => {
            increase_brightness();
        }
        _ => {}
    }
}

fn increase_brightness() {
    let max_brightness = brightness::get_max_brightness();
    let mut brightness = brightness::get_brightness();
    if brightness == max_brightness {
        return;
    }
    brightness += 5;
    if brightness > max_brightness {
        brightness = max_brightness;
    }
    brightness::set_brightness(brightness);
}

fn decrease_brightness() {
    let mut brightness = brightness::get_brightness();
    if brightness == 0 {
        return;
    }
    if brightness < 5 {
        brightness = 0;
    } else {
        brightness -= 5;
    }
    brightness::set_brightness(brightness);
}

fn main() {
    let max_brightness = brightness::get_max_brightness();
    println!("Max brightness: {}", max_brightness);
    println!("Brightness: {}", brightness::get_brightness());
    brightness::set_brightness(200);
    HeadphoneMonitor::start();
    let mut event_stream = InputEventStream::new("platform-odroidgo2-joypad-event-joystick", true);
    let receiver = GamepadMonitor::start(event_stream.event_file.try_clone().ok().unwrap());
    loop {
        let event = receiver.recv();
        if let Ok(event) = event {
            if event.event_code != EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY1) {
                event_stream.send(event);
                continue;
            }
            printlog!("hotkey down");
            let mut command_given = false;
            let mut key_states: HashMap<EV_KEY, Instant> = HashMap::new();
            loop {
                let hk_event = receiver.try_recv();
                process_events(&mut key_states);
                if hk_event.is_err() {
                    thread::sleep(time::Duration::from_millis(10));
                    continue;
                }
                let hk_event = hk_event.unwrap();
                match hk_event.event_code {
                    EventCode::EV_KEY(EV_KEY::BTN_TRIGGER_HAPPY1) => {
                        if !command_given {
                            event_stream.send(event);
                            event_stream.send(hk_event);
                        }
                        break;
                    }
                    EventCode::EV_SYN(_) => {
                        continue;
                    }
                    EventCode::EV_KEY(v) => {
                        command_given = true;
                        if hk_event.value == 1 {
                            key_states.insert(v, Instant::now());
                        } else {
                            key_states.remove(&v);
                        }
                    }
                    _ => {
                        event_stream.send(hk_event);
                    }
                }
            }
            printlog!("hotkey up");
        }
    }
}
