    pub struct AX25 {
        src_callsign : String,
        dest_callsign : String,
        encoded: Vec<u8>,
    }
    
    impl AX25 {
        pub fn new(src:&str, dest:&str)->AX25{
            AX25{
                src_callsign : src.to_string(),
                dest_callsign : dest.to_string(),
                encoded: Vec::new(),
            }
        }

        fn add_callsign(&mut self, callsign:&str)
        {
            let mut s:Vec<&str> = callsign.split('-').collect();
            if s.len()< 2{
                s.push("0")
            }
            let ssid: u8 = s[1].parse().unwrap();

            for ch in format!("{:>6}", s[0]).chars(){
                self.encoded.push((ch as u8)<<1);
            }
            self.encoded.push((48+ssid) << 1);
        }

        fn mark_end_of_callsigns(&mut self)
        {
            let v = self.encoded.pop().unwrap();
            self.encoded.push(v | 0x01);
        }

        fn add_crc(&mut self)
        {
            let mut crc:u16 = 0xffff;
            for ch in &self.encoded {
                crc ^= *ch as u16;
                for _ in 0..8{
                    if crc & 0x01 != 0{
                        crc = (crc>>1) ^ 0x8404;
                    }
                    else
                    {
                        crc >>= 1;
                    }
                }

            }
            self.encoded.push(!(crc & 0xff) as u8);
            self.encoded.push(!((crc>>8) & 0xff) as u8);
        }

        pub fn frame(&mut self, msg:&str) -> Vec<u8>{
            self.encoded = Vec::new();
            self.add_callsign(&self.dest_callsign.to_string());
            self.add_callsign(&self.src_callsign.to_string());
            self.add_callsign("WIDE1-1");
            self.add_callsign("WIDE2-1");
            self.mark_end_of_callsigns();
            self.encoded.push(0x03); // Control: 0x03 = APRS-UI frame
            self.encoded.push(0xF0); // Protocol ID: 0xF0 = no layer 3 data
            self.encoded.extend(msg.as_bytes());
            self.add_crc();
            self.encoded.to_vec()
        }
    }
 