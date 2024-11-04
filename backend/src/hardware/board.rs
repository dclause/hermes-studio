use anyhow::Result;
use hermes_five::hardware::Board as HermesBoard;
use hermes_five::io::IO;
use serde::{Deserialize, Serialize};

use crate::devices::Device;
use crate::impl_entity;
use crate::utils::database::{ArcDb, Database};
use crate::utils::entity::{Entity, Id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: Id,
    pub name: String,
    pub model: BoardType,
    #[serde(flatten)]
    pub inner: HermesBoard,
    #[serde(skip)]
    pub connected: bool,
}

impl_entity!(Board, {
    fn post_load(&mut self, _: &Database) -> Result<()> {
        // Reset connection state on load.
        self.connected = false;
        Ok(())
    }
    // Delete all associated devices.
    fn post_delete(&mut self, database: &mut Database) -> Result<()> {
        let devices = database.list::<Device>()?;
        for (_, device) in devices {
            if device.hid == self.id {
                database.delete::<Device>(device.id)?;
            }
        }
        Ok(())
    }
});

impl Board {
    pub fn open(mut self, database: &ArcDb) -> Result<Self> {
        self.inner = self.inner.blocking_open()?;
        self.connected = self.inner.is_connected();

        // Initialize properly the inner device value because now that board is open(), the
        // handshake as given us the hardware board configuration, which lets us properly initialize
        // our devices.
        let devices = database.write().list::<Device>()?;
        for (_, mut device) in devices {
            if device.hid == self.id {
                device.inner.set_hardware(&self.inner)?;
                device.save(&database)?;
            }
        }

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
pub enum RaspberryType {
    ZERO,
    #[allow(non_camel_case_types)]
    ZERO_W,
    TWO,
    THREE,
    FOUR,
    FIVE,
    #[default]
    OTHER,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum BoardType {
    Arduino(ArduinoType),
    RaspberryPi(RaspberryType),
    #[default]
    Unknown,
}
