#[cfg(feature = "display")]
use std::fmt::{Display, Formatter};

#[cfg(feature = "debug")]
use std::fmt::{Debug};


#[derive(Hash, Eq, PartialEq, PartialOrd)]
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

#[cfg(feature = "display")]
impl Display for TSID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars = String::with_capacity(13);
        let number = self.number as usize;

        chars.push(ALPHABET[(number >> 60) & 0b11111]);
        chars.push(ALPHABET[(number >> 55) & 0b11111]);
        chars.push(ALPHABET[(number >> 50) & 0b11111]);
        chars.push(ALPHABET[(number >> 45) & 0b11111]);
        chars.push(ALPHABET[(number >> 40) & 0b11111]);
        chars.push(ALPHABET[(number >> 35) & 0b11111]);
        chars.push(ALPHABET[(number >> 30) & 0b11111]);
        chars.push(ALPHABET[(number >> 25) & 0b11111]);
        chars.push(ALPHABET[(number >> 20) & 0b11111]);
        chars.push(ALPHABET[(number >> 15) & 0b11111]);
        chars.push(ALPHABET[(number >> 10) & 0b11111]);
        chars.push(ALPHABET[(number >> 5) & 0b11111]);
        chars.push(ALPHABET[number & 0b11111]);

        return f.write_str(chars.as_str());
    }
}

#[cfg(feature = "debug")]
impl Debug for TSID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl Into<u64> for TSID {
    fn into(self) -> u64 {
        self.number
    }
}

impl From<u64> for TSID {
    fn from(value: u64) -> Self {
        TSID::new(value)
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

        assert!(id1 != id2, "Ids shouldnt be equal {} {}", id1, id2);
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
}
