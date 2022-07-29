use rusty_pipe_viewer::{args::Args, read, stats, write};
use std::io::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_bytes = 0;
    
    'main: loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break 'main,
            Ok(x) => x,
            Err(_) => break 'main
        };
        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)? {
            break 'main;
        }
    }

    stats::stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}
