use slot_stream::{SlotSummary, parse_slots, summarize_slots};
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

    for slot in &slots {
        println!("slot: {slot}");
    }

    let result: SlotSummary = summarize_slots(&slots).unwrap();
    println!("{result}");

    ExitCode::SUCCESS
}
