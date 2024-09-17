use std::sync::Arc;

use hermes_five::errors::Error;
use hermes_five::protocols::{Hardware, Message, Protocol};
use log::trace;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaspiProtocol {
    /// Indicates whether the protocol as gone through the handshake properly.
    #[serde(skip)]
    connected: bool,
    /// The base-protocol attributes.
    #[serde(skip)]
    hardware: Arc<RwLock<Hardware>>,
}

impl RaspiProtocol {
    /// Constructs a new `RaspiProtocol` instance for controlling a raspberry board.
    pub fn new<P: Into<String>>() -> Self {
        Self {
            connected: false,
            hardware: Arc::new(RwLock::new(Hardware::default())),
        }
    }
}

#[typetag::serde]
impl Protocol for RaspiProtocol {
    /// Retrieve the internal hardware.
    fn get_hardware(&self) -> &Arc<RwLock<Hardware>> {
        &self.hardware
    }

    /// Checks if the communication is opened using the underlying protocol.
    fn is_connected(&self) -> bool {
        self.connected
    }

    /// Sets the protocol inner connected indicator.
    fn set_connected(&mut self, status: bool) {
        self.connected = status;
    }

    /// Opens communication.
    ///
    /// # Returns
    /// * `Ok(())` if successful.
    /// * `Err(Error)` - unused.
    fn open(&mut self) -> Result<(), Error> {
        // Perform handshake.
        self.set_connected(false);
        self.hardware.write().firmware_name = String::from("No firmware");
        self.hardware.write().firmware_version = String::from("1.0.0");
        self.hardware.write().protocol_version = String::from("1.0.0");
        // @todo implement properly
        trace!("Raspi port is now opened");

        self.set_connected(true);
        Ok(())
    }

    /// Gracefully shuts down the communication.
    ///
    /// # Returns
    /// * `Ok(())` if successful.
    /// * `Err(Error)` - unused.
    fn close(&mut self) -> Result<(), Error> {
        self.connected = false;
        Ok(())
    }

    /// Write bytes to the internal connection. For more details see [`std::io::Write::write`].
    ///
    /// # Arguments
    /// * `buf` - The data to write.
    ///
    /// # Returns
    /// * `Ok(())` if all bytes were successfully written.
    /// * `Err(Error)` if there was an issue writing data.
    ///
    /// # Notes
    /// This function blocks until the write operation is complete. Ensure proper error handling in calling code.
    fn write(&mut self, buf: &[u8]) -> Result<(), Error> {
        todo!()
    }

    /// Reads from the internal connection. For more details see [`std::io::Read::read_exact`].
    ///
    /// # Arguments
    /// * `buf` - The buffer to fill with read data.
    ///
    /// # Returns
    /// * `Ok(())` if the buffer was filled successfully.
    /// * `Err(Error)` if there was an issue reading data.
    ///
    /// # Notes
    /// This function blocks until the buffer is filled or an error occurs. Ensure proper error handling in calling code.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        todo!()
    }
}
