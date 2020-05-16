extern crate clap;

use clap::{Arg, App};
use std::io;
use std::io::prelude::*;

fn main() {
    let matches = App::new("aprs-message")
                          .version("1.0")
                          .author("Ido Roseman <ido.roseman@gmail.com>")
                          .about("create an aprs message packet")
                          .arg(Arg::with_name("dest")
                               .short("d")
                               .long("to")
                               .help("callsign of recipiant")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("message")
                               .help("message to send")
                               .index(1))
                          .get_matches();

  let dest = matches.value_of("dest").unwrap();
  let text = matches.value_of("message").unwrap_or("");

  if text.is_empty()
  {
     let stdin = io::stdin();
     for line in stdin.lock().lines() {
        println!(":{:<9}:{}", dest, line.unwrap());
    }
  }
  else
  {
    println!(":{:<9}:{}", dest, text);
  }
}
