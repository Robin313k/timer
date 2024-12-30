use actually_beep::beep_with_hz_and_millis;
use std::env;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("Usage:\ntimer [minutes as Int] [seconds as Int]");

        std::process::exit(0);
    }

    let mut minutes: i32 = args[1].trim().parse().expect("Invalid input");
    let mut seconds: i32 = if args.len() > 2 {
        args[2].parse().unwrap_or_else(|_| {
            println!("Invalid input for seconds. Please provide an integer.");
            std::process::exit(1);
        })
    } else {
        0 // Default value for seconds
    };

    while seconds > 59 {
        seconds = seconds - 60;
        minutes = minutes + 1;
    }


    print!("{} minutes and {} seconds left", minutes, seconds);

    io::stdout().flush().unwrap();


    for m in 0..minutes + 1 {
        for s in 0..seconds {
            sleep(Duration::from_secs(1));

            print!("\r{} minutes and {} seconds left", minutes, seconds - (s + 1));
        
            io::stdout().flush().unwrap();
        }
        
        if minutes > 0 {
            minutes = minutes - 1;
            seconds = 60;
        }
    }

    println!("\nYour time is up!");

    let frequenz_hz = 528;
    let less_than_a_second_ms = 800;
    beep_with_hz_and_millis(frequenz_hz, less_than_a_second_ms).unwrap();
}

