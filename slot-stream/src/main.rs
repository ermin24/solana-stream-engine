use slot_stream::parse_slots;
use std::io::{self, Read};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut input = String::new();
    if let Err(error) = io::stdin().read_to_string(&mut input) {
        eprintln!("Error reading stdin: {error}");
        return ExitCode::FAILURE;
    }

    let slots = match parse_slots(&input) {
        Ok(slots) => slots,
        Err(error) => {
            eprintln!("failed to parse slot: {error}");
            return ExitCode::FAILURE;
        }
    };

    let slots_length = slots.len();
    for slot in &slots {
        println!("slot: {slot}");
    }
    println!("count: {slots_length}");
    match slot_stream::slot_range(&slots) {
        Some((min, max)) => {
            println!("min: {min}");
            println!("max: {max}")
        }
        None => {
            println!("no slots available");
        }
    }

    ExitCode::SUCCESS
}
