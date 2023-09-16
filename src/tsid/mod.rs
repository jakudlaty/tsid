pub struct TSID {
    number: u64,
}

impl TSID {
    pub(crate) fn new(number: u64) -> Self {
        Self {
            number
        }
    }
}

impl ToString for TSID {
    fn to_string(&self) -> String {
        let mut chars: [char; 13] = [' '; 13];
        let number = self.number as usize;

        let alphabet = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z'];

        chars[0x00] = alphabet[(number >> 60) & 0b11111];
        chars[0x01] = alphabet[(number >> 55) & 0b11111];
        chars[0x02] = alphabet[(number >> 50) & 0b11111];
        chars[0x03] = alphabet[(number >> 45) & 0b11111];
        chars[0x04] = alphabet[(number >> 40) & 0b11111];
        chars[0x05] = alphabet[(number >> 35) & 0b11111];
        chars[0x06] = alphabet[(number >> 30) & 0b11111];
        chars[0x07] = alphabet[(number >> 25) & 0b11111];
        chars[0x08] = alphabet[(number >> 20) & 0b11111];
        chars[0x09] = alphabet[(number >> 15) & 0b11111];
        chars[0x0a] = alphabet[(number >> 10) & 0b11111];
        chars[0x0b] = alphabet[(number >> 5) & 0b11111];
        chars[0x0c] = alphabet[number & 0b11111];

        return chars.iter().collect()
    }
}