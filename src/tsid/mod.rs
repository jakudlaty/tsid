pub mod conversions;

#[cfg(feature = "display")]
pub mod display;

#[cfg(feature = "debug")]
pub mod debug;

#[cfg(feature = "bson")]
pub mod bson;

#[cfg(feature = "serde")]
pub mod serde;

#[derive(Hash, Eq, PartialEq, PartialOrd, Copy, Clone)]
pub struct TSID {
    number: u64,
}

const ALPHABET: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J',
    'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
];

impl TSID {
    pub(crate) fn new(number: u64) -> Self {
        Self { number }
    }

    pub fn number(&self) -> u64 {
        self.number
    }
}

#[cfg(test)]
mod tests {
    use crate::TSID;

    #[test]
    fn tsid_should_have_small_size() {
        assert_eq!(
            8,
            std::mem::size_of::<TSID>(),
            "TSID should have size of exactly 8 bytes"
        );
    }

    #[test]
    #[cfg(feature = "display")]
    fn should_implement_ordering() {
        let id1 = TSID::new(0);
        let id2 = TSID::new(10);

        assert!(
            id1 < id2,
            "Id2:{} should be greater than Id1:{} because it was created later",
            id2.to_string(),
            id1
        );
    }

    #[test]
    #[cfg(feature = "display")]
    fn string_representations_should_be_also_ordered() {
        let id1 = TSID::new(9);
        let id2 = TSID::new(10);

        assert!(
            id1.to_string() < id2.to_string(),
            "Id2:{} should be greater than Id1:{} because it was created later",
            id2.to_string(),
            id1
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serialize_to_human_readable_form() {
        let id1 = TSID::new(496830748901259172);
        println!(
            "{}",
            serde_json::to_string_pretty(&id1).expect("Unable to serialize")
        )
    }

    #[test]
    #[cfg(feature = "bson")]
    fn serialize_to_bson() {
        let id1 = TSID::new(496830748901259172);
        println!("{}", bson::doc! {"id": id1 })
    }
}
