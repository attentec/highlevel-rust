extern crate clap;

// Use clap-rs for argument parsing.
use clap::{Arg, App, SubCommand};

// This line is needed for the json! macro
#[macro_use]
extern crate serde_json;

// Because we want to use macros from error_chain, the
// crate import must be done in the main file.
#[macro_use]
extern crate error_chain;

// Must import this for #[derive(Deserialize)]
#[macro_use]
extern crate serde_derive;

// Import our own modules
mod config;
mod net;
mod list_jobs;
mod list_nodes;

// Struct to store command line arguments
struct Args {
    config: String,
    subcommand: Option<String>,
}

// Parse command line arguments using Clap and
// convert to our own Args struct.
// This is sligthly messy due to lack of
// ability to refer to objects
fn parse_args() -> Args {
    let args = App::new("Command Line Util")
                        .version("1.0")
                        .author("Mikael Silvén <mikael.silven@attentec.se>")
                        .about("Does awesome things!")
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Sets a custom config file")
                            .takes_value(true)
                            .default_value("config.toml"))
                        .subcommand(SubCommand::with_name("list-jobs")
                            .about("List all jobs on server"))
                        .subcommand(SubCommand::with_name("list-nodes")
                            .about("List all nodes connected to this master"))
                        .get_matches();
    
    // We can use expect() here because we have declared a default value
    let config_arg = args.value_of("config").expect("config");

    return Args {
        config: String::from(config_arg),
        // map() to convert the Option<&str> to an Option<String>
        subcommand: args.subcommand_name().map(|s| String::from(s)),
    };
}


fn main() {
    let args = parse_args();

    // Use & to lend args.config
    let config = config::read_config(&args.config).expect("Could not read config!");
    
    // Mandatory Hello statement 
    println!("Hello, {}!", config.jenkins.server);

    if let Some(subcommand) = args.subcommand {
        match subcommand.as_str() {
            "list-jobs" => list_jobs::execute(&config),
            "list-nodes" => list_nodes::execute(&config),
            _ => panic!("Unknown command"),
        }
    }
}