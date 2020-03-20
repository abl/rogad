pub struct GPIO {
    path: String,
}
impl GPIO {
    pub fn new(pin: u16) -> GPIO {
        GPIO {
            path: format!("/sys/class/gpio/gpio{}/value", pin),
        }
    }

    fn read(&self) -> u8 {
        let state_bytes: Vec<u8> = std::fs::read(self.path.as_str()).unwrap();
        // Currently handles digital GPIO only.
        let state = state_bytes[0] - 48;
        printlog!("read:{} state:{}", self.path, state);

        state
    }

    pub fn read_bool(&self) -> bool {
        // If the first (currently only) digit is zero, the value is zero.
        self.read() != 0
    }
}
