use serde::{Serialize, Serializer};
use crate::TSID;

#[cfg(feature = "serde")]
impl Serialize for TSID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if serializer.is_human_readable() {
            serializer.serialize_str(self.to_string().as_str())
        } else {
            serializer.serialize_u64(self.number)
        }
    }
}