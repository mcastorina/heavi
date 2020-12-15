use clap_v3::{App, Arg};
use regex::bytes::Regex;
use std::fs::File;
use std::io::{self, Write};

const BUF_SIZE: usize = 4096;

fn main() {
    let matches = App::new("heavi")
        .version("0.1.0")
        .author("miccah <m.castorina93@gmail.com>")
        .about("Output text until a certain point")
        .arg(
            Arg::with_name("PATTERN")
                .help("Pattern to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILE")
                .help("File to output")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("invert-match")
                .short('v')
                .help("Invert (start outputting at that point)"),
        )
        .get_matches();

    // Get arguments
    // ------------------------------------------------------------
    // Calling .unwrap() is safe here because "PATTERN" is required
    let pattern = matches.value_of("PATTERN").unwrap();

    let mut stream = match matches.value_of("FILE") {
        Some(file) => Box::new(File::open(file).unwrap()) as Box<dyn io::Read>, // FIXME: unwrap
        None => Box::new(io::stdin()) as Box<dyn io::Read>,
    };

    let invert = matches.is_present("invert-match");

    // Iterate through stream
    // ------------------------------------------------------------
    let re = Regex::new(pattern).unwrap(); // FIXME: unwrap
    let mut stdout = io::stdout();

    let mut dbuf = [0; BUF_SIZE];
    let mut n = stream.read(&mut dbuf).unwrap(); // FIXME: unwrap
    loop {
        if n == 0 {
            break;
        }
        if let Some(m) = re.find(&dbuf) {
            let sect = &dbuf[..m.start()];
            if let Some(idx) = sect.iter().rev().position(|&r| r == 10) {
                stdout.write(&dbuf[..m.start() - idx]).unwrap(); // FIXME: unwrap
            }
            break;
        }
        stdout.write(&dbuf[..BUF_SIZE / 2]).unwrap(); // FIXME: unwrap

        dbuf.copy_within(BUF_SIZE / 2.., 0);
        n = stream.read(&mut dbuf[BUF_SIZE / 2..]).unwrap(); // FIXME: unwrap
    }
}
