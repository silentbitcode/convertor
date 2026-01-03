use std::io;

fn main() {
    loop { //control flow
        println!("Enter a number you want to convert, or type q to quit");
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
        let value = value.trim();
        if value.eq_ignore_ascii_case("q") {
            break;
        }
        let x: i64 = match value.parse() {
            Ok(v) => v,
            Err(_) => { //error handling
                println!("Invalid number, Try Again!");
                continue;
            }
        };
        println!("\n choose a conversion type");
        println!("1, Metres to Kilometres");
        println!("2, Kilometres to Metres");
        println!("3, Seconds to Hours");
        println!("4, Hours to Seconds");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
//match..
        match choice.trim() {
            "1" => println!("{}metres = {} km", x, m_to_km(x)),
            "2" => println!("{}km = {} metres", x, km_to_m(x)),
            "3" => println!("{}sec = {} hrs", x, sec_to_hrs(x)),
            "4" => println!("{}hrs = {} secs", x, hrs_to_sec(x)),
            _ => println!("Invalid Choice"),
        }
    }
    println!("GoodBye!")
}

//Converts metres to kilometres
fn m_to_km(x: i64) -> i64 {
    //1km== 1000Metres
    x / 1000
}
//converts kilometres to metres
fn km_to_m(x: i64) -> i64 {
    x * 1000
}
//converts seconds to hours
fn sec_to_hrs(x: i64) -> i64 {
    //1 hour == 3600 seconds
    x / 3600
}
//converts hours to seconds
fn hrs_to_sec(x: i64) -> i64 {
    x * 3600
}
