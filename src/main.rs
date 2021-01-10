use clap_v3::{App, Arg};
use std::fs::File;
use std::io::{self, Write};

fn main() -> Result<(), heavi::HeaviError> {
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

    let stream = match matches.value_of("FILE") {
        Some(file) => Box::new(File::open(file)?) as Box<dyn io::Read>,
        None => Box::new(io::stdin()) as Box<dyn io::Read>,
    };

    let invert = matches.is_present("invert-match");
    let mut stdout = io::stdout();

    // Call relevant functions
    // ------------------------------------------------------------
    if invert {
        heavi::heavi_inv(stream, &mut stdout, pattern)?;
    } else {
        heavi::heavi(stream, &mut stdout, pattern)?;
    }

    stdout.flush()?;
    Ok(())
}
