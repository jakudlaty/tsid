use std::fmt::Formatter;
use crate::TSID;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};

impl Serialize for TSID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.to_string().as_str())
        } else {
            serializer.serialize_u64(self.number)
        }
    }
}


impl<'de> Deserialize<'de> for TSID {
    fn deserialize<D>(deserializer: D) -> Result<TSID, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(TSIDIntVisitor)
    }
}

struct TSIDIntVisitor;

impl<'de> Visitor<'de> for TSIDIntVisitor {
    type Value = TSID;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an integer or string representation of TSID")
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: Error {
        Ok(TSID::new(v as u64))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: Error {
        Ok(TSID::new(v))
    }


    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> where E: Error {
        todo!()
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error> where D: Deserializer<'de> {
        todo!()
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> where E: Error {
        todo!()
    }
}