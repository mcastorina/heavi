use clap_v3::{App, Arg};
use heavi::{Heavi, HeaviError, HeaviParser};
use std::fs::File;
use std::io::{self, BufReader, Write};

fn main() -> Result<(), HeaviError> {
    let matches = App::new("heavi")
        .version("0.1.0")
        .author("miccah <m.castorina93@gmail.com>")
        .about("Output text starting at a certain point")
        .arg(
            Arg::with_name("PATTERN")
                .help("Pattern to search for")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("FILE")
                .help("File to read")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("invert-match")
                .long("invert")
                .short('v')
                .help("Invert (stop outputting at that point)"),
        )
        .arg(
            Arg::with_name("byte-mode")
                .long("byte-mode")
                .short('b')
                .help("Byte processing instead of line processing"),
        )
        .arg(
            Arg::with_name("inclusive")
                .long("inclusive")
                .short('i')
                .help("Output the matched line or bytes"),
        )
        .get_matches();

    // Get arguments
    // ------------------------------------------------------------
    // Calling .unwrap() is safe here because "PATTERN" is required
    let pattern = matches.value_of("PATTERN").unwrap();

    let stream = match matches.value_of("FILE") {
        Some(file) => Box::new(File::open(file)?) as Box<dyn io::Read>,
        None => Box::new(io::stdin()) as Box<dyn io::Read>,
    };

    let invert = matches.is_present("invert-match");
    let byte_mode = matches.is_present("byte-mode");
    let inclusive = matches.is_present("inclusive");
    let mut stdout = io::stdout();

    // Call relevant functions
    // ------------------------------------------------------------
    Heavi {
        line_mode: !byte_mode,
        invert: invert,
        inclusive: inclusive,
        output: &stdout,
    }
    .parse(BufReader::new(stream), pattern)?;

    stdout.flush()?;
    Ok(())
}
