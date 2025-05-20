use crate::tsid::{ParseErrorReason, TsidError, REVERSE, TSID};

impl From<TSID> for u64 {
    fn from(val: TSID) -> Self {
        val.number
    }
}

impl From<u64> for TSID {
    fn from(value: u64) -> Self {
        TSID::new(value)
    }
}

impl TryFrom<&str> for TSID {
    type Error = TsidError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 13 {
            return Err(TsidError::ParseError(ParseErrorReason::InvalidLength));
        }
        if !value.as_bytes().iter().all(|c| REVERSE.contains_key(&c)) {
            return Err(TsidError::ParseError(ParseErrorReason::InvalidCharacters));
        }
        
        let chars = value.as_bytes();

        let mut number = 0u64;
        number |= REVERSE[&chars[0x00]] << 60;
        number |= REVERSE[&chars[0x01]] << 55;
        number |= REVERSE[&chars[0x02]] << 50;
        number |= REVERSE[&chars[0x03]] << 45;
        number |= REVERSE[&chars[0x04]] << 40;
        number |= REVERSE[&chars[0x05]] << 35;
        number |= REVERSE[&chars[0x06]] << 30;
        number |= REVERSE[&chars[0x07]] << 25;
        number |= REVERSE[&chars[0x08]] << 20;
        number |= REVERSE[&chars[0x09]] << 15;
        number |= REVERSE[&chars[0x0a]] << 10;
        number |= REVERSE[&chars[0x0b]] << 5;
        number |= REVERSE[&chars[0x0c]];

        Ok(TSID::new(number))
    }
}

#[cfg(test)]
mod tests {
    use crate::tsid::TSID;

    #[test]
    fn should_convert_from_u64() {
        let val = 496830748901259230u64;
        let id = TSID::from(val);
        assert_eq!("0DS8RXW6W0DYY", id.to_string())
    }

    #[test]
    fn converted_should_match_original() {
        use rand::random;
        const LOOP_MAX: usize = 1000;

        for _ in 0..LOOP_MAX {
            let number = random::<u64>();
            let bytes0 = number.to_be_bytes();

            let string0 = TSID::new(number).to_string();
            let tsid = TSID::try_from(string0.as_str()).unwrap();
            let bytes1 = tsid.number().to_be_bytes();

            assert_eq!(bytes0, bytes1);
        }
    }

    #[test]
    fn test_is_valid() {
        // Null/empty cases
        assert!(!TSID::try_from("").is_ok());

        // Case variations
        assert!(TSID::try_from("0123456789ABC").is_ok()); // All upper case
        assert!(TSID::try_from("0123456789abc").is_ok()); // All lower case
        assert!(TSID::try_from("0123456789AbC").is_ok()); // Mixed case

        // Length checks
        assert!(!TSID::try_from("0123456789AB").is_ok()); // Too short (12)
        assert!(!TSID::try_from("0123456789ABCC").is_ok()); // Too long (14)

        // Letter validation
        assert!(TSID::try_from("0123456789ABi").is_ok()); // Letter I
        assert!(TSID::try_from("0123456789ABl").is_ok()); // Letter L  
        assert!(TSID::try_from("0123456789ABo").is_ok()); // Letter O
        assert!(TSID::try_from("0123456789ABu").is_err()); // Letter U not allowed

        // Special characters
        assert!(TSID::try_from("0123456789AB#").is_err()); // Special char
        assert!(TSID::try_from("0123456789AB\u{3617}").is_err()); // Unicode char

        // Random valid cases
        for _ in 0..1000 {
            let tsid = TSID::new(rand::random::<u64>());
            assert!(TSID::try_from(tsid.to_string().as_str()).is_ok());
        }
    }
}

