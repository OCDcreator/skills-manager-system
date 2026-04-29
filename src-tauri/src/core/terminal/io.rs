use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn spawn_output_pump(
    mut reader: Box<dyn Read + Send>,
    output: Arc<Mutex<String>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut buffer = [0_u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(size) => {
                    let text = String::from_utf8_lossy(&buffer[..size]);
                    output.lock().unwrap().push_str(&text);
                }
                Err(_) => break,
            }
        }
    })
}
