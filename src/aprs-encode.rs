extern crate clap;

use clap::{Arg, App};
use std::io::prelude::*;

mod aprs;
use aprs::ax25;
use aprs::modem;

fn main() {
    let matches = App::new("aprs-encode")
                          .version("1.0")
                          .author("Ido Roseman <ido.roseman@gmail.com>")
                          .about("takes an aprs message and create an ax.25 wave file")
                          .arg(Arg::with_name("source")
                                .short("s")
                                .long("src")
                                .help("callsign-ssid of sender")
                                .takes_value(true)
                                .required(true))
                          .arg(Arg::with_name("destination")
                                .short("d")
                                .long("dest")
                                .help("callsign-ssid of recipient")
                                .takes_value(true)
                                .required(true))
                          .arg(Arg::with_name("message")
                               .help("message to send")
                               .index(1))
                          .get_matches();

  let src = matches.value_of("source").unwrap();
  let dest = matches.value_of("destination").unwrap_or("APE6UB");
  let text = matches.value_of("message").unwrap_or("");

  let mut frame = ax25::AX25::new(src, dest);
  let mut encoder = modem::Modem::new("sine.wav", src, dest);
  encoder.write_frame(&frame.frame(text))

}

