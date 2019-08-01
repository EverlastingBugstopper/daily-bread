extern crate clap;
use clap::App;

fn main() {
    App::new("daily-bread")
        .version("0.1.0")
        .about("Emails your notes and commit messages to you at the end of the day")
        .author("Avery Harnish")
        .get_matches();
}
