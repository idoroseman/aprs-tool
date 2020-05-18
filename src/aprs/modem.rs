use std::f32::consts::PI;
use std::i16;

static BAUD:i32 = 1200;
static lowFreq:f32 = 1200.0;
static highFreq:f32 = 2200.0;
static bitDuration:f32 = 1.0 / (BAUD as f32);
static sample_rate:u32 = 48000;

static preamble_length:i32 = 128;
static postamble_length:i32 = 64;
static flags_before:i32 = 32;
static flags_after:i32 = 32;


pub struct Modem {
	bitcount: i32,
	is_high:bool,
    phase:f32,
	writer: hound::WavWriter<std::io::BufWriter<std::fs::File>>,
}

impl Modem {

    pub fn new(filename:&str)-> Modem {
		let spec = hound::WavSpec {
			channels: 1,
			sample_rate: sample_rate,
			bits_per_sample: 16,
			sample_format: hound::SampleFormat::Int,
		};

		Modem{
			bitcount:0,
			is_high:false,
			phase:0.0,
			writer:hound::WavWriter::create(filename, spec).unwrap(),
		}
	}

	fn write_freq(&mut self , freq:f32, duration:f32){
		let amplitude = i16::MAX as f32;
        let delta = (freq * 2.0 * PI)/ (sample_rate as f32);
		for _t in (0 .. ((sample_rate as f32)*duration) as i32).map(|x| x as f32 ) {
			self.phase += delta;
			let sample = (self.phase).sin();
			self.writer.write_sample((sample * amplitude) as i16).unwrap();
		}
	}

	fn write_tone(&mut self , bit:bool){
		if !bit  {
		  self.write_freq( lowFreq, bitDuration);
		} else {
		  self.write_freq( highFreq, bitDuration);
		}
	}

	fn write_bit(&mut self, val:u8, bit_stuffing:bool){
		let bit:bool = val & 0x01 != 0;
		if bit_stuffing 
		{
			if self.bitcount >= 5 
				{
					self.is_high = !self.is_high;
					self.write_tone(self.is_high);
					self.bitcount = 0;
				}
		}
		else
		{
				self.bitcount = 0;
		}
		
		if  bit   // Stay with same frequency, but only for a max of 5 in a row
		{
				self.bitcount+=1; 		  
		}
		else // 0 means swap frequency
		{
				self.is_high = !self.is_high; 		  
				self.bitcount = 0;
		}
		self.write_tone(bit);
	}
	
	fn write_byte(&mut self , character:u8, bit_stuffing:bool){
		let mut ch = character;
		for _ in 0..8 {
			self.write_bit( ch & 1, bit_stuffing);
			ch >>= 1;
		}
	}
	
	pub fn write_frame(&mut self, msg: Vec<u8>){

	  self.bitcount = 0;
	  self.is_high = false;
	  // Write preamble
	  for _ in 0..preamble_length
		{
			self.write_tone(false);
		}
  
		for _ in 0..flags_before
		{
			self.write_byte( 0x7E, false);
		}
	  
		// Create and write actual data
		for c in msg
		{
			self.write_byte( c, true);
		}
  
		for _ in 0..flags_after
		{
			self.write_byte( 0x7E, true);
		}
  
	  // Write postamble
	  for _ in 0..postamble_length
		{
			self.write_tone( false);
		}
  }
}
