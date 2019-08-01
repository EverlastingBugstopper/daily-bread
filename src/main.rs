extern crate clap;
use clap::{App, Arg, SubCommand};

extern crate chrono;
use chrono::{Datelike, Local};

use directories::ProjectDirs;

use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;
use std::path::PathBuf;

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
        let now = Local::now();
        let year_name = now.year();
        let year = now.format("%y");
        let month = format!("{:02}", now.month());
        let month_name = now.format("%B");
        let day = format!("{:02}", now.day());
        if let Some(proj_dirs) = ProjectDirs::from("com", "EverlastingBugstopper", "daily-bread") {
            let mut dir = PathBuf::from(proj_dirs.config_dir());
            dir.push(format!("{}/{}", year_name, month_name));
            let mut path = PathBuf::from(dir.clone());
            path.push(format!("{}-{}-{}.txt", month, day, year));
            fs::create_dir_all(dir)?;
            let mut file = OpenOptions::new().create(true).append(true).open(path)?;
            write!(file, "{}\n", note)?;
        }
    } else if let Some(_) = matches.subcommand_matches("config") {
        println!("Config command run!");
    }
    Ok(())
}
