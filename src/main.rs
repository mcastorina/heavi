use clap_v3::{App, Arg};
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), heavi::HeaviError> {
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
    let mut stdout = io::stdout();

    // Call relevant functions
    // ------------------------------------------------------------
    match (byte_mode, invert) {
        (true, true) => heavi::heavi_inv(stream, &mut stdout, pattern)?,
        (true, false) => heavi::heavi(stream, &mut stdout, pattern)?,
        (false, true) => heavi::heavi_line_inv(stream, &mut stdout, pattern)?,
        (false, false) => heavi::heavi_line(stream, &mut stdout, pattern)?,
    };

    stdout.flush()?;
    Ok(())
}
