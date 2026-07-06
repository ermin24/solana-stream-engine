pub fn slot_range(slots: &[u64]) -> Option<(u64, u64)> {
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
