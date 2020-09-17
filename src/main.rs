extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("yang")
                          .version("1.0")
                          .author("Carlos Wu.")
                          .about("Does awesome things")
                          .subcommand(SubCommand::with_name("gen")
                                      .about("Generating rust source code through Yang files")
                                      .version("1.0")
                                      .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("gen") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}