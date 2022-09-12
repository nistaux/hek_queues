use std::time::Duration;
use enigo::{Enigo, Key, KeyboardControllable};

fn main() {
    let mut t = 0;
    while t < 8 {

        let mut enigo = Enigo::new();

        enigo.key_down(Key::Space);
        enigo.key_up(Key::Space);
        std::thread::sleep(Duration::from_secs(1));

        t = t+1;
    }
}
