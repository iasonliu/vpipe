use std::io::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn stats_loop(silent: bool,  quit: Arc<AtomicBool>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // TODO: receive the vector of bytes
        let buffer: Vec<u8> = Vec::new(); // so we can complie
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        // TODO: send vector to write loop
        if quit.load(Ordering::SeqCst)  {
            break;
        }
    }
    eprintln!();
    Ok(())
}
