use bson::Bson;
use crate::TSID;

impl From<TSID> for Bson {
    fn from(value: TSID) -> Self {
        Bson::Int64(value.number as i64)
    }
}

#[cfg(test)]
mod tests {
    use crate::{TSID};

    #[test]
    fn serialize_to_bson() {
        let id1 = TSID::new(496830748901259172);
        println!("{}", bson::doc! {"id": id1 })
    }
}
