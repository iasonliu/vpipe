use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(help = "Read from a file instead of stdin", default_value = "")]
    pub infile: String,
    #[clap(
        short = 'o',
        long = "outfile",
        takes_value = true,
        help = "Write to stdout instead of stdout",
        default_value = ""
    )]
    pub outfile: String,
    #[clap(short = 's', long = "silent", help = "silent output")]
    pub silent: bool,
}

impl Args {
  pub fn get_args() -> Self {
    let args = Args::parse();
    let silent = if args.silent {
      true
    } else {
      !env::var("PPVR_SILENT").unwrap_or_default().is_empty()
    };

    Self{
      infile: args.infile,
      outfile: args.outfile,
      silent
    }
  }
}
