fn main() {
    let slots: Vec<u64> = vec![348_100_001, 348_100_002, 348_100_005];
    // let slots: Vec<u64> = vec![];
    let slots_length = slots.len();
    for slot in &slots {
        println!("slot: {slot}");
    }
    println!("count: {slots_length}");
    match slot_range(&slots) {
        Some((min, max)) => {
            println!("min: {min}");
            println!("max: {max}")
        }
        None => {
            println!("no slots available");
        }
    }
}
fn slot_range(slots: &[u64]) -> Option<(u64, u64)> {
    if slots.is_empty() {
        return None;
    }
    let min = slots.iter().min().unwrap();
    let max = slots.iter().max().unwrap();
    Some((*min, *max))
}

#[cfg(test)]
mod tests {
    use crate::slot_range;

    #[test]
    fn test_slot_range() {
        assert_eq!(
            slot_range(&[348_100_001, 348_101_111]),
            Some((348_100_001, 348_101_111))
        );
    }
    #[test]
    fn test_slot_range_empty() {
        assert_eq!(slot_range(&[]), None);
    }
}
