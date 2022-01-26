use clap::{App, Arg};
use std::path::Path;
use yang::cmd::generate::parse_yang;

fn main() {
    let matches = App::new("yang")
        .version("1.0")
        .about("Yang CLI")
        .author("Carlos Wu. <wuanxiang@outlook.com>")
        .subcommand(
            App::new("gen")
                .about("Generate source code of programming language through Yang files")
                .version("1.0")
                .arg(
                    Arg::new("language")
                        .long("language")
                        .short('l')
                        .takes_value(true)
                        .possible_values(&["Rust", "Java", "Golang"])
                        .default_value("Rust")
                        .help("Set programming language. If not, the default is Rust"),
                )
                .arg(
                    Arg::new("dir")
                        .value_name("Yang files directory")
                        .help("Set the input directory to use.")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("gen") {
        println!(
            "Value for language: {}",
            matches.value_of("language").unwrap()
        );
        println!("Value for dir: {}", matches.value_of("dir").unwrap());
        if let Err(err) = parse_yang(Path::new(matches.value_of("dir").unwrap())) {
            println!("error {}", err);
        }
    }
}
