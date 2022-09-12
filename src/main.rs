use std::time::Duration;
use enigo::{Enigo, Key, KeyboardControllable};

fn main() {
    println!("\n\nImitating random movement keyboard events...\n\n");
    println!("To Quit: Click the X on this window or press Ctrl+C in the console");

    let mut t = 0;

    const W: Key = Key::Layout('w');
    while t < 8 {

        let mut enigo = Enigo::new();

        enigo.key_down(W);
        std::thread::sleep(Duration::from_secs(1));
        enigo.key_up(W);
        std::thread::sleep(Duration::from_secs(1));

        t = t+1;
    }
}
