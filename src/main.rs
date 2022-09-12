use std::time::Duration;
use std::thread;
use enigo::{Enigo, Key, KeyboardControllable};
use rand::{thread_rng, Rng, rngs::ThreadRng};

const W: Key = Key::Layout('w');
const A: Key = Key::Layout('a');
const S: Key = Key::Layout('s');
const D: Key = Key::Layout('d');
const SPACE: Key = Key::Space;

fn main() {
    println!("\n\nImitating random movement keyboard events...\n\n");
    println!("To Quit: Click the X on this window or press Ctrl+C in the console\n\n ");

    let mut rng = thread_rng();
    let mut enigo = Enigo::new();

    loop {
        let op = rng.gen_range(1..6);

        match op {
            1 => { do_movement(Movement::Jump, &mut rng, &mut enigo); }
            2 => { do_movement(Movement::JumpBack, &mut rng, &mut enigo); }
            3 => { do_movement(Movement::JumpForward, &mut rng, &mut enigo); }
            4 => { do_movement(Movement::JumpLeft, &mut rng, &mut enigo); }
            5 => { do_movement(Movement::JumpRight, &mut rng, &mut enigo); }
            _ => {
                println!("Weird, that isnt supposed to happen...");
            }
        }
        //thread::sleep(Duration::new(rng.gen_range(63..612), rng.gen_range(1..500000)));
        thread::sleep(Duration::new(rng.gen_range(1..6), rng.gen_range(1..500000)));
    }
}

fn do_movement(op: Movement, rng: &mut ThreadRng, enigo: &mut Enigo) {
    match op {
        Movement::Jump => {
            enigo.key_down(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(SPACE);
        }
        Movement::JumpLeft => {
            enigo.key_down(A);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_down(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(A);
        }
        Movement::JumpRight => {
            enigo.key_down(D);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_down(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(D);
        }
        Movement::JumpBack => {
            enigo.key_down(S);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_down(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(S);
        }
        Movement::JumpForward => {
            enigo.key_down(W);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_down(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(SPACE);
            thread::sleep(Duration::from_millis(rng.gen_range(10..76)));
            enigo.key_up(W);
        }
        _ => {
            println!("Weird, that isnt supposed to happen...");
        }
    }
}

enum Movement {
    Jump,
    JumpLeft,
    JumpRight,
    JumpBack,
    JumpForward,
    SquigleAllAround,
    SquigleLeft,
    SquigleRight,
    SquigleForward,
    SquigleBack
}
