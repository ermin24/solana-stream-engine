fn main() {
    // let slots: Vec<u64> = vec![348_100_001, 348_100_002, 348_100_005];
    let slots: Vec<u64> = vec![];
    if slots.is_empty() {
        println!("no slots available");
        return;
    }
    let slots_length = slots.len() as u8;
    let slots_max = max_slots(&slots);
    let slots_min = min_slots(&slots);
    for slot in slots {
        println!("slot: {slot}");
    }
    println!("count: {slots_length}");
    println!("max: {slots_max}");
    println!("min: {slots_min}")
}

fn max_slots(slots: &[u64]) -> u64 {
    slots.iter().max().copied().expect("slots is empty")
}
fn min_slots(slots: &[u64]) -> u64 {
    slots.iter().min().copied().expect("slots is empty")
}
