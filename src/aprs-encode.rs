extern crate clap;

use clap::{Arg, App};
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, BufRead};

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
                                .takes_value(true))
							.arg(Arg::with_name("input")
                                .short("i")
                                .long("input")
                                .help("input filename")
								.takes_value(true))
							.arg(Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .help("output filename")
								.takes_value(true))
                        //    .arg(Arg::with_name("filenme")
                        //        .help("message to send")
                        //        .index(1))
                          .get_matches();

  let src = matches.value_of("source").unwrap();
  let dest = matches.value_of("destination").unwrap_or("APE6UB");
  let input = matches.value_of("input");
  let output = matches.value_of("output").unwrap_or("aprs.wav");

  let mut frame = ax25::AX25::new(src, dest);
  let mut encoder = modem::Modem::new(output);
  
  let reader: Box<BufRead> = match input {
		None => Box::new(BufReader::new(io::stdin())),
		Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap()))
	};

  for line in reader.lines() {
	encoder.write_frame(frame.frame(&line.unwrap()));
  }

}

