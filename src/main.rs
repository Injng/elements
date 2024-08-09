fn main() {
    // get args and check for at least 2
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    // see if file exists
    let filename = &args[1];
    if !std::path::Path::new(filename).exists() {
        eprintln!("File not found: {}", filename);
        std::process::exit(1);
    }

    // open file and read into string
    let contents = std::fs::read_to_string(filename).expect("Failed to read file");
    println!("{}", contents);
}
