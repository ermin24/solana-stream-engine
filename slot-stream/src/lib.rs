use std::num::ParseIntError;
pub fn slot_range(slots: &[u64]) -> Option<(u64, u64)> {
    if slots.is_empty() {
        return None;
    }
    let min = slots.iter().min().unwrap();
    let max = slots.iter().max().unwrap();
    Some((*min, *max))
}

pub fn parse_slots(input: &str) -> Result<Vec<u64>, ParseIntError> {
    // implementation
    input.split_whitespace().map(|s| s.parse()).collect()
}

#[cfg(test)]
mod tests {
    use crate::{parse_slots, slot_range};

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
    #[test]
    fn test_parse_slots() {
        assert_eq!(parse_slots("42"), Ok(vec![42]));
        assert_eq!(parse_slots("10 20\n30"), Ok(vec![10, 20, 30]));
        assert_eq!(parse_slots(""), Ok(Vec::new()));
    }

    #[test]
    fn test_parse_slots_invalid_input() {
        let result = parse_slots("10 invalid 10");
        assert!(result.is_err())
    }
}
