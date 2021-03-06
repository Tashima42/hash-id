extern crate clap;
use clap::{Command, Arg};

use hash_id::{run, Config};

fn main() {
    let matches = Command::new("Hash Identifier")
        .version("0.1.0")
        .author("Pedro Tashima <pedrotashima@protonmail.com>")
        .about("Identify different types of hashes")
        .arg(
            Arg::new("hash")
                .short('h') 
                .long("hash")
                .value_name("STRING")
                .help("Hash value to be identified")
                .takes_value(true),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("File containing hashes (each one in a line)")
                .takes_value(true),
        )
        .get_matches();

    let hash = matches.value_of("hash").unwrap_or_default();
    let file = matches.value_of("file").unwrap_or_default();

    let config = Config::new(hash.to_string(), file.to_string());
    run(config);
}
