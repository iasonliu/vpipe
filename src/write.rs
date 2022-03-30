use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn write_loop(outfile: &str,  quit: Arc<AtomicBool>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // TODO: recevie vector from stats thread
        let buffer: Vec<u8> = Vec::new(); // so we can complie
        {
            if quit.load(Ordering::SeqCst) {
                break;
            }
        }
        if let Err(e) = writer.write_all(&buffer) {
            match e.kind() {
                ErrorKind::BrokenPipe => {
                    // stop the program cleanly
                    return Ok(());
                }
                _ => return Err(e),
            }
        };
    }
    Ok(())
}
