use clap::Parser;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "Read from a file instead of stdin", default_value = "")]
    infile: String,
    #[clap(
        short = 'o',
        long = "outfile",
        takes_value = true,
        help = "Write to stdout instead of stdout",
        default_value = ""
    )]
    outfile: String,
    #[clap(short = 's', long = "silent", help = "silent output")]
    silent: bool,
}
const CHUNK_SIZE: usize = 16 * 1024;
fn main() -> Result<()> {
    let args = Args::parse();
    let infile = args.infile;
    let outfile = args.outfile;
    let silent = if args.silent {
        true
    } else {
        !env::var("PPVR_SILENT").unwrap_or_default().is_empty()
    };
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            match e.kind() {
                ErrorKind::BrokenPipe => break,
                _ => return Err(e),
            }
        };
    }
    Ok(())
}
