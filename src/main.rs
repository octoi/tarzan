use std::{thread, time};
use enigo::{Enigo, Key, KeyboardControllable};

fn generate_text(content: String, delay: u32) {
    let time_delay = time::Duration::from_secs(delay as u64);
    thread::sleep(time_delay);

    let mut enigo = Enigo::new();

    for (idx, char) in content.to_lowercase().chars().enumerate() {
        let ch = content.chars().nth(idx).unwrap();

        if ch == '\n' {
            enigo.key_click(Key::Return);
            continue;
        }

        if ch.is_uppercase() {
            enigo.key_click(Key::CapsLock);
        }

        enigo.key_click(Key::Layout(char));

        if ch.is_uppercase(){
            enigo.key_click(Key::CapsLock);
        }

    }
}

fn main() {

}