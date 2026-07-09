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
    input.split_whitespace().map(|s| s.parse()).collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct SlotSummary {
    pub count: usize,
    pub first: u64,
    pub last: u64,
    pub min: u64,
    pub max: u64,
}

pub fn summarize_slots(slots: &[u64]) -> Option<SlotSummary> {
    if slots.is_empty() {
        return None;
    };
    let count = slots.len();
    let min = slots.iter().min().copied()?;
    let max = slots.iter().max().copied()?;
    let first = slots.first().copied()?;
    let last = slots.last().copied()?;
    Some(SlotSummary {
        count,
        first,
        last,
        max,
        min,
    })
}

#[cfg(test)]
mod tests {
    use crate::{SlotSummary, parse_slots, slot_range, summarize_slots};

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
    fn test_parse_slots_single() {
        assert_eq!(parse_slots("42"), Ok(vec![42]));
    }

    #[test]
    fn test_parse_slots_multi() {
        assert_eq!(parse_slots("1 2 3 4 5"), Ok(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_parse_slots_mixed_whitespace() {
        assert_eq!(parse_slots("10 20\n30"), Ok(vec![10, 20, 30]));
    }

    #[test]
    fn test_parse_slots_empty() {
        assert_eq!(parse_slots(""), Ok(Vec::new()));
    }

    #[test]
    fn test_parse_slots_invalid_input() {
        let result = parse_slots("10 invalid 10");
        assert!(result.is_err())
    }
    #[test]
    fn test_summarize_slots_empty() {
        assert_eq!(summarize_slots(&[]), None);
    }
    #[test]
    fn test_summarize_slots_single() {
        let single = 348_100_001;
        let only = SlotSummary {
            count: 1,
            max: single,
            min: single,
            first: single,
            last: single,
        };
        let summary = match summarize_slots(&[single]) {
            Some(s) => s,
            None => return,
        };
        assert_eq!(summary, only);
    }

    #[test]
    fn test_summarize_slots_multi_a() {
        let multi = SlotSummary {
            count: 4,
            max: 40,
            min: 4,
            first: 40,
            last: 12,
        };
        let summary = match summarize_slots(&[40, 4, 24, 12]) {
            Some(s) => s,
            None => return,
        };
        assert_eq!(summary, multi)
    }
}
