extern crate clap;
use clap::{App, Arg, SubCommand};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;

fn main() -> Result<(), Error> {
    let matches = App::new("daily-bread")
        .version("0.1.0")
        .about("Emails your notes and commit messages to you at the end of the day")
        .author("Avery Harnish")
        .subcommand(
            SubCommand::with_name("note")
                .about("Appends a note to your daily note file")
                .arg(
                    Arg::with_name("input")
                        .help("the note to take")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("config").help("Sets your configuration"))
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("note") {
        let note = matches.value_of("input").unwrap();
        let path = "./notes/date.txt";
        // TODO make directory if doesn't exist
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        write!(file, "{}\n", note)?;
    } else if let Some(_) = matches.subcommand_matches("config") {
        println!("Config command run!");
    }
    Ok(())
}
