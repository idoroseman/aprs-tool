extern crate clap;

use clap::{Arg, App};


fn base91enc(value:i32, length: i32) -> String {
    let mut rv = String::new();
    let mut val:i32 = value;
    for _ in 0..length{
        rv.push(std::char::from_u32((val % 91 + 33) as u32).unwrap());
        val /= 91  ;
    }
    rv.chars().rev().collect()
}


fn main() {
    let matches = App::new("aprs-location")
                          .version("1.0")
                          .author("Ido Roseman <ido.roseman@gmail.com>")
                          .about("create an aprs location packet")
                          .arg(Arg::with_name("lat")
                                .help("Latitude")
                                .index(1)
                                .required(true))
                          .arg(Arg::with_name("lon")
                                .help("Longitude")
                                .index(2)
                                .required(true))
                            .arg(Arg::with_name("alt")
                                .short("a")
                                .long("alt")
                                .help("Altitude")
                                .takes_value(true)
                                )
                            .arg(Arg::with_name("symbol")
                                .short("s")
                                .long("symbol")
                                .help("Symbol")
                                .takes_value(true)
                                .default_value("O")
                                )
                        .arg(Arg::with_name("comment")
                               .help("Comment")
                               .index(3)
                               )
                          .get_matches();

    let lat = matches.value_of("lat").unwrap().parse::<f32>().unwrap();
    let lon = matches.value_of("lon").unwrap().parse::<f32>().unwrap();
    let alt = matches.value_of("alt").unwrap_or("0").parse::<f32>().unwrap();
    let symbol = matches.value_of("symbol").unwrap_or_default();
    let comment = matches.value_of("comment").unwrap_or("");

    // see Chapter 9: Compressed Position Report Data Formats
    // Convert the min.dec coordinates to APRS compressed format
    let aprs_lat:i32 = (380926.0 * (90.0 - lat)) as i32;
    let aprs_lon:i32 = (190463.0 * (180.0 + lon)) as i32;
    //  convert from meter to feet
    let aprs_alt:i32 = (alt * 32808.0 / 10000.0) as i32;

    print!("!"); // real-time report without timestamp
    print!("/"); // Symbol Table Identifier
    print!("{}{}", base91enc(aprs_lat, 4), base91enc(aprs_lon, 4));
    print!("{}", symbol); // symbol code: BALLOON
    if aprs_alt > 0 {
        print!("/A={:06}", aprs_alt);
    }
    print!("{}", comment);
    println!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
       let lat:f32 = 32.063331;
       let lon:f32 = 34.872165;
       let aprs_lat:i32 = (380926.0 * (90.0 - lat)) as i32;
       let aprs_lon:i32 = (190463.0 * (180.0 + lon)) as i32;
       assert_eq!(aprs_lat, 22069584);
       assert_eq!(aprs_lon, 40925196);
    }

    #[test]
    fn test_base91enc() {
       
        assert_eq!(base91enc(22069583, 4), ">;(r");
        assert_eq!(base91enc(40925197, 4), "W=&I");

    }
}