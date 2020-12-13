use clap_v3::{App, Arg};

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

    // Calling .unwrap() is safe here because "PATTERN" is required
    println!(
        "Using input pattern: {}",
        matches.value_of("PATTERN").unwrap()
    );

    if let Some(file) = matches.value_of("FILE") {
        println!("Using input file: {}", file);
    }

    if matches.is_present("invert-match") {
        println!("Inverting");
    }
}
