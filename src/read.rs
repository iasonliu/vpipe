use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn read_loop(infile: &str,  quit: Arc<AtomicBool>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        //TODO: send this buffer to the stats thread
        let _ = Vec::from(&buffer[..num_read]);
    }

    //TODO: send an an empty buffer to the stats thread
    quit.store(true, Ordering::Relaxed);
    Ok(())
}
