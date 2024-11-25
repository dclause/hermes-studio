mod board;
mod expander;
mod pca9685;
mod raspi;

use crate::utils::entity::Id;
pub use board::*;
pub use expander::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "id")]
pub enum HardwareType {
    Board(Id),
    Expander(Id),
}

impl Deref for HardwareType {
    type Target = Id;

    fn deref(&self) -> &Self::Target {
        match self {
            HardwareType::Board(id) => id,
            HardwareType::Expander(id) => id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::HardwareType;

    #[test]
    fn test_serialize() {
        let id = HardwareType::Board(5);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, r#"{"type":"Board","id":5}"#);
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"type":"Board","id":5}"#;
        let id = serde_json::from_str::<HardwareType>(&json);
        assert!(id.is_ok());
        let id = id.unwrap();
        matches!(id, HardwareType::Board(5));
        matches!(*id, 5);
    }
}
