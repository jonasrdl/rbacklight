use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader};
use std::process;

const MAX_PATH: &str = "/sys/class/backlight/intel_backlight/max_brightness";
const VAL_PATH: &str = "/sys/class/backlight/intel_backlight/brightness";
const VERSION: &str = "0.0.1";

fn bget(path: &str) -> Result<i32, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let val = reader.lines().next().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Empty file"))??;
    Ok(val.parse::<i32>().unwrap())
}

fn bset(path: &str, val: i32) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().write(true).open(path)?;
    write!(file, "{}", val)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1].starts_with('-') {
        match args[1].as_str() {
            "-h" => {
                println!("usage: {} [-hv] [VALUE]", args[0]);
                process::exit(1);
            }
            "-v" => {
                println!("{} {}", args[0], VERSION);
                process::exit(1);
            }
            _ => (),
        }
    }

    let max = bget(MAX_PATH).unwrap();
    let mut val = bget(VAL_PATH).unwrap();

    if args.len() >= 2 {
        let n = &args[1];

        let numeric_val = n.parse::<i32>();
        let set = numeric_val.is_ok();

        if set {
            val = numeric_val.unwrap();
        } else if n.ends_with('%') {
            let percentage = n.trim_end_matches('%').parse::<i32>().unwrap();
            val = (max * percentage) / 100;
        }

        val = val.max(0).min(max);
        bset(VAL_PATH, val).unwrap();
    } else {
        println!("{}", val);
    }
}
