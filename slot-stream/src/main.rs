fn main() {
    let slots: Vec<u64> = vec![348_100_001, 348_100_002, 348_100_005];
    // let slots: Vec<u64> = vec![];
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
}
