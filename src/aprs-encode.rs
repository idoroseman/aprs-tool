extern crate clap;

use clap::{Arg, App};
use std::io::prelude::*;
use std::f32::consts::PI;
use std::i16;
use std::io;

static baud:i32 = 1200;
static lowFreq:f32 = 1200.0;
static highFreq:f32 = 2200.0;
static bitDuration:f32 = 1.0 / (baud as f32);
static preamble_length:i32 = 128;
static postamble_length:i32 = 64;
static flags_before:i32 = 32;
static flags_after:i32 = 32;

fn write_freq<W:io::Write + io::Seek>(writer: &mut hound::WavWriter<W> , freq:f32, duration:f32){
    for t in (0 .. (44100.0*duration) as i32).map(|x| x as f32 / 44100.0) {
        let sample = (t * freq * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
}

fn write_tone<W:io::Write + io::Seek>(writer: &mut hound::WavWriter<W> , bit:u8){
  if bit == 0 {
    write_freq(writer, lowFreq, bitDuration)
  } else {
    write_freq(writer, highFreq, bitDuration)
  }
}

fn write_bit<W:io::Write + io::Seek>(writer: &mut hound::WavWriter<W> , bit:u8, bitStuffing:bool){
	if bitStuffing 
	{
	    if bitcount >= 5 
		  {
			  bit = !bit;
			  write_tone(writer, 1);
			  bitccount = 0;
		  }
	}
	else
	{
		  bitcount = 0;
	}
	
	if  bit != 0  // Stay with same frequency, but only for a max of 5 in a row
	{
		  bitcount+=1; 		  
	}

	else // 0 means swap frequency
	{
		  bit = !bit; 		  
		  bitcount = 0;
	}
	write_tone(writer, bit);
}

fn write_byte<W:io::Write + io::Seek>(writer: &mut hound::WavWriter<W> , ch:u8, bitStuffing:bool){
  for i in 0..8 {
		write_bit(writer, ch & 1, bitStuffing);
		ch >>= 1;
	}
}

fn write_frame<W:io::Write + io::Seek>(writer: &mut hound::WavWriter<W> , msg:&str){

  	// Write preamble
    for i in 0..preamble_length
	  {
		  write_tone(writer, 0);
	  }

	  for i in 0..flags_before
	  {
		  write_byte(writer, 0x7E, false);
	  }
	
	  // Create and write actual data
	  for c in msg.chars()
	  {
		  write_byte(writer, c, true);
	  }

	  for i in 0..flags_after
	  {
		  write_byte(writer, 0x7E, true);
	  }

    // Write postamble
    for i in 0..postamble_length
	  {
		  write_tone(writer, 0);
	  }
}

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
  let spec = hound::WavSpec {
      channels: 1,
      sample_rate: 44100,
      bits_per_sample: 16,
      sample_format: hound::SampleFormat::Int,
  };
  let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
  write_frame(&mut writer, &text)
}

