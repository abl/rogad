use std::process::{Command, Stdio};

pub struct Amixer {}
impl Amixer {
    const HEADPHONE: &'static str = "HP";
    const SPEAKER: &'static str = "SPK";

    fn _update(&self, target: &str) {
        Command::new("/usr/bin/amixer")
            .arg("cset")
            .arg("name='Playback Path'")
            .stdout(Stdio::null())
            .arg(target)
            .output()
            .expect("Unable to change mixer output.");
    }

    pub fn set_headphone(&self) {
        printlog!("Changing mixer output to headphones");
        self._update(Amixer::HEADPHONE);
    }

    pub fn set_speaker(&self) {
        printlog!("Changing mixer output to speaker");
        self._update(Amixer::SPEAKER);
    }
}
pub static AMIXER: Amixer = Amixer {};
