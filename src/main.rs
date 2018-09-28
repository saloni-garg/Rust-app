use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;

#[macro_use] extern crate nickel;
use nickel::Nickel;

extern crate chrono;
use chrono::{DateTime,Local};

extern crate clap;
use clap::{App,Arg};

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn entry_formatted(filename: &String, bytes: &[u8]) -> io::Result<()> {
    let mut file = try!(OpenOptions::new().
                        append(true).
                        write(true).
                        create(true).
                        open(filename));
    try!(file.write_all(bytes));
    Ok(())
}

fn log_time(filename: &String) -> io::Result<String> {
    let entry = formatted_time_entry();	
    {
        let bytes = entry.as_bytes();

        try!(entry_formatted(filename, &bytes));
    }
    Ok(entry)
}

fn do_log_time(logfile_path: &String, auth_token: &Option<String>) -> String {
    match log_time(logfile_path) {
        Ok(entry) => format!("Entry Logged: {}", entry),
        Err(e) => format!("Error: {}", e)
    }
}

fn main() {
    let matches = App::new("simple-log").version("v0.0.1")
        .arg(Arg::with_name("LOG FILE")
             .short("l")
             .long("logfile")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("AUTH TOKEN")
             .short("t")
             .long("token")
             .takes_value(true))
        .get_matches();

    let logfile_path = matches.value_of("LOG FILE").unwrap().to_string();
    let auth_token = match matches.value_of("AUTH TOKEN") {
        Some(str) => Some(str.to_string()),
        None => None
    };

    let mut server = Nickel::new();
    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time(&logfile_path, &auth_token)
        }
    });

    server.listen("127.0.0.1:6767");

}
