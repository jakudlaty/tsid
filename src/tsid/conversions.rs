use crate::TSID;
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
    use crate::{TSID};

    #[test]
    #[cfg(feature = "display")]
    fn should_convert_from_u64() {
        let val = 496830748901259230u64;
        let id = TSID::from(val);
        assert_eq!("0DS8RXW6W0DYY", id.to_string())
    }
}
