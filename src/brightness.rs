pub fn get_max_brightness() -> u8 {
    let contents = std::fs::read_to_string(
        "/sys/devices/platform/backlight/backlight/backlight/max_brightness",
    )
    .expect("Could not read max brightness");
    contents.trim().parse().unwrap()
}

pub fn get_brightness() -> u8 {
    let contents =
        std::fs::read_to_string("/sys/devices/platform/backlight/backlight/backlight/brightness")
            .expect("Could not read brightness");
    contents.trim().parse().unwrap()
}

pub fn set_brightness(value: u8) {
    std::fs::write(
        "/sys/devices/platform/backlight/backlight/backlight/brightness",
        value.to_string(),
    )
    .expect("Could not set brightness");
}
