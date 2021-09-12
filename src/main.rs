use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // Changed the fn name to new because it's a convention for naming constructors.
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough parameters.");
        }

        // Pull out individual args.
        // Index 0 is the binary so we skip that.
        // We clone the strings here to not take ownership.
        // Not performant but easiest to do right now.
        let query = args[1].clone();
        let filename = args[2].clone();

        // Make a new config to relate the data.
        // We returned a tuple before but that doesn't show relationship.
        Ok(Config{query, filename})
    }
}

fn main() {
    // ===Collect the args into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Call the function to parse.
    // unwrap_or_else() returns Ok variant or runs this closure if error.
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        // This terminates program with status code passed in.
        process::exit(1);
    });

    // Return all the args.
    println!("===args: {:?}", args);
    
    // Returns the saved args.
    println!("===query: {}", config.query);
    
    // Print out the filename and contents.
    println!("===filename: {}", config.filename);

    // This opens the file and returns Result type.
    // Use expect() to handle Ok and Err.
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // Prints out the contents of the file.
    println!("===contents:\n{}", contents);

    // ==Output running `cargo run the poem.txt`
    // ===
    // ["target\\debug\\minigrep.exe", "the", "poem.txt"]
    // ===
    // Searching for the
    // ===
    // In file poem.txt
    // With text:
    // I'm nobody! Who are you?
    // Are you nobody, too?
    // Then there's a pair of us - don't tell!
    // They'd banish us, you know.

    // How dreary to be somebody!
    // How public, like a frog
    // To tell your name the livelong day
    // To an admiring bog!
}
