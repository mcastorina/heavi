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
        let mut dbuf_len = n;

        // find the match
        while dbuf_len > 0 {
            let idx = match re.find(&dbuf) {
                Some(m) => Some(m.end()),
                None => None,
            };
            if let Some(idx) = idx {
                // shift out the match
                // |_match_|__dbuf_len-idx__|
                //      idx^        dbuf_len^
                dbuf.copy_within(idx.., 0);
                n = stream.read(&mut dbuf[dbuf_len - idx..]).unwrap(); // FIXME: unwrap

                // |__dbuf_len-idx__|_n_|
                dbuf_len = dbuf_len - idx + n;
                break;
            }
            // iterate through the file
            if dbuf_len == BUF_SIZE {
                // |__buf1__|__buf2__|
                dbuf.copy_within(BUF_SIZE / 2.., 0);
                n = stream.read(&mut dbuf[BUF_SIZE / 2..]).unwrap(); // FIXME: unwrap

                // |__buf2__|__n__|
                dbuf_len = BUF_SIZE / 2 + n;
            } else {
                // no more data to read in
                dbuf = [0; BUF_SIZE];
                dbuf_len = 0;
            }
        }

        // find the newline
        while dbuf_len > 0 {
            let idx = dbuf.iter().position(|&r| r == 10);
            if let Some(idx) = idx {
                let idx = idx + 1;
                // shift out the newline
                // |_CR_|__dbuf_len-idx__|
                //   idx^        dbuf_len^
                dbuf.copy_within(idx.., 0);
                n = stream.read(&mut dbuf[BUF_SIZE - idx..]).unwrap(); // FIXME: unwrap

                // |__dbuf_len-idx__|_n_|
                dbuf_len = dbuf_len - idx + n;
                break;
            }
            n = stream.read(&mut dbuf).unwrap(); // FIXME: unwrap
            dbuf_len = n;
        }

        // print the rest of the file
        while dbuf_len > 0 {
            stdout.write(&dbuf[..dbuf_len]).unwrap(); // FIXME: unwrap
            dbuf_len = stream.read(&mut dbuf).unwrap(); // FIXME: unwrap
        }
    }
}
