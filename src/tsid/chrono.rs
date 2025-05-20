use crate::consts::RANDOM_BITS;
use crate::TSID;
use chrono::{DateTime, Duration, TimeZone, Utc};

impl TSID {
    /// Returns a chrono::DateTime<Utc>
    pub fn timestamp(&self) -> DateTime<Utc> {
        let dt = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        dt + Duration::milliseconds((self.number >> RANDOM_BITS) as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::TSID;
    use chrono::{TimeZone, Utc};

    #[test]
    fn should_set_timestamp_to_tsid_epoch() {
        let id_min = TSID::new(0);
        assert_eq!(
            id_min.timestamp(),
            Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap()
        )
    }

    #[test]
    fn should_set_timestamp_to_max_timestamp_value() {
        let id_max = TSID::new(u64::MAX);
        assert_eq!(
            id_max.timestamp(),
            chrono::DateTime::parse_from_rfc3339("2159-05-15T07:35:11.103Z").unwrap()
        )
    }

    #[test]
    fn test_tsid_max_256() {
        test_tsid_max(20000, 16384, 8);
    }

    #[test]
    fn test_tsid_max_1024() {
        test_tsid_max(10000, 4096, 10);
    }

    #[test]
    fn test_tsid_max_4096() {
        test_tsid_max(10000, 1024, 12);
    }

    fn test_tsid_max(max_loop: usize, max_tsid: usize, node_bits: u8) {
        let mut list = Vec::with_capacity(max_loop);

        let mut factory = crate::TsidFactory::with_node_bits(node_bits, 0);
        for _ in 0..max_loop {
            // can generate up to 4096 per msec
            list.push(factory.create());
        }

        let mut n = 0;
        let mut prev_time = 0;
        for tsid in list {
            let time = tsid.timestamp().timestamp_millis();
            if time != prev_time {
                n = 0;
            }
            n += 1;
            prev_time = time;
            assert!(n <= max_tsid, "Too many TSIDs: {}", n);
        }
    }
}
