    pub struct AX25 {
        src_callsign : String,
        src_ssid : i32,
        dest_callsign : String,
        dest_ssid : i32,
    }
    
    impl AX25 {
        pub fn new(src:&str, dest:&str)->AX25{
            let mut s:Vec<&str> = src.split('-').collect();
            let mut d:Vec<&str> = dest.split('-').collect();
            if s.len()< 2{
                s.push("0")
            }
            if d.len() <2 {
                d.push("0")
            }
            AX25{
                src_callsign : String::from(s[0]),
                src_ssid : s[1].parse().unwrap(),
                dest_callsign : String::from(d[0]),
                dest_ssid : d[1].parse().unwrap(),
            }
        }

        pub fn frame(&mut self, msg:&str) -> String{
            String::from(msg)
        }
    }
 