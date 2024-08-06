use anyhow::Result;
use hermes_five::Board as InnerBoard;
use serde::{Deserialize, Serialize};

use crate::impl_entity;
use crate::utils::entity::{Entity, Id};

type ShallowBoard = Board;

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: Id,
    pub name: String,
    pub model: BoardType,
    #[serde(flatten)]
    pub inner: InnerBoard,
    pub connected: bool,
}

impl_entity!(Board, {
    fn post_load(&mut self) {
        // Reset connection state on load.
        self.connected = false;
    }
});

impl Board {
    pub fn open(mut self) -> Result<Self> {
        self.inner = self.inner.blocking_open()?;
        self.connected = self.inner.is_connected();
        Ok(self)
    }
    pub fn close(mut self) -> Result<Self> {
        self.inner = self.inner.close();
        self.connected = false;
        Ok(self)
    }
}

// ########################################

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum ArduinoType {
    NANO,
    UNO,
    MEGA,
    #[default]
    OTHER,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum BoardType {
    Arduino(ArduinoType),
    #[default]
    Unknown,
}
