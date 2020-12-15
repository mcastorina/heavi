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
    if !invert {
        while n != 0 {
            if let Some(m) = re.find(&dbuf) {
                let sect = &dbuf[..m.start()];
                // last newline before pattern
                if let Some(idx) = sect.iter().rev().position(|&r| r == 10) {
                    stdout.write(&dbuf[..m.start() - idx]).unwrap(); // FIXME: unwrap
                }
                break;
            }
            stdout.write(&dbuf[..BUF_SIZE / 2]).unwrap(); // FIXME: unwrap

            // iterate through the file
            dbuf.copy_within(BUF_SIZE / 2.., 0);
            n = stream.read(&mut dbuf[BUF_SIZE / 2..]).unwrap(); // FIXME: unwrap
        }
    } else {
        // find the match
        while dbuf[0] != 0 {
            let idx = match re.find(&dbuf) {
                Some(m) => Some(m.end()),
                None => None,
            };
            if let Some(idx) = idx {
                // shift out the match
                dbuf.copy_within(idx.., 0);
                n = stream.read(&mut dbuf[BUF_SIZE - idx..]).unwrap(); // FIXME: unwrap
                if n < idx {
                    // fill with 0
                    for i in BUF_SIZE-idx..BUF_SIZE {
                        dbuf[i] = 0;
                    }
                }
                break;
            }
            // iterate through the file
            dbuf.copy_within(BUF_SIZE / 2.., 0);
            n = stream.read(&mut dbuf[BUF_SIZE / 2..]).unwrap(); // FIXME: unwrap
        }

        // find the newline
        while dbuf[0] != 0 {
            let idx = dbuf.iter().position(|&r| r == 10);
            if let Some(idx) = idx {
                // shift out the newline
                dbuf.copy_within(idx+1.., 0);
                n = stream.read(&mut dbuf[BUF_SIZE-(idx+1)..]).unwrap(); // FIXME: unwrap
                if n < idx+1 {
                    // fill with 0
                    for i in BUF_SIZE-(idx+1)..BUF_SIZE {
                        dbuf[i] = 0;
                    }
                }
                break;
            }
            n = stream.read(&mut dbuf).unwrap(); // FIXME: unwrap
            if n < BUF_SIZE {
                // fill with 0
                for i in n..BUF_SIZE {
                    dbuf[i] = 0;
                }
            }
        }

        // print the rest of the file
        stdout.write(&dbuf).unwrap(); // FIXME: unwrap
        while n != 0 {
            n = stream.read(&mut dbuf).unwrap(); // FIXME: unwrap
            stdout.write(&dbuf[..n]).unwrap(); // FIXME: unwrap
        }
    }
}
