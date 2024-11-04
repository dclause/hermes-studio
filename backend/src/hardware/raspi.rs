use std::fmt::{Display, Formatter};
use std::sync::Arc;

use hermes_five::errors::Error;
use hermes_five::io::{IoData, IoProtocol, PinModeId, IO};
use hermes_five::utils::Range;
use log::trace;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RaspiIo {
    /// Indicates whether the protocol as gone through the handshake properly.
    #[serde(skip)]
    connected: bool,
    /// The base-protocol attributes.
    #[serde(skip)]
    data: Arc<RwLock<IoData>>,
}

impl RaspiIo {
    /// Constructs a new `RaspiProtocol` instance for controlling a raspberry board.
    #[allow(dead_code)]
    pub fn new<P: Into<String>>() -> Self {
        Self {
            connected: false,
            data: Arc::new(RwLock::new(IoData::default())),
        }
    }

    /// Sets the protocol inner connected indicator.
    fn set_connected(&mut self, status: bool) {
        self.connected = status;
    }
}

#[typetag::serde]
impl IoProtocol for RaspiIo {
    fn open(&mut self) -> Result<(), Error> {
        // Perform handshake.
        self.set_connected(false);
        self.data.write().firmware_name = String::from("No firmware");
        self.data.write().firmware_version = String::from("1.0.0");
        self.data.write().protocol_version = String::from("1.0.0");
        // @todo implement properly
        trace!("Raspi port is now opened");

        self.set_connected(true);
        Ok(())
    }
    fn close(&mut self) -> Result<(), Error> {
        self.connected = false;
        Ok(())
    }

    fn report_analog(&mut self, _: u8, _: bool) -> Result<(), Error> {
        todo!()
    }
    fn report_digital(&mut self, _: u8, _: bool) -> Result<(), Error> {
        todo!()
    }
    fn sampling_interval(&mut self, _: u16) -> Result<(), Error> {
        todo!()
    }
}

impl IO for RaspiIo {
    fn get_io(&self) -> &Arc<RwLock<IoData>> {
        todo!()
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn set_pin_mode(&mut self, _: u8, _: PinModeId) -> Result<(), Error> {
        todo!()
    }

    fn digital_write(&mut self, _: u8, _: bool) -> Result<(), Error> {
        todo!()
    }

    fn analog_write(&mut self, _: u8, _: u16) -> Result<(), Error> {
        todo!()
    }

    fn digital_read(&mut self, _: u8) -> Result<bool, Error> {
        todo!()
    }

    fn analog_read(&mut self, _: u8) -> Result<u16, Error> {
        todo!()
    }

    fn servo_config(&mut self, _: u8, _: Range<u16>) -> Result<(), Error> {
        todo!()
    }

    fn i2c_config(&mut self, _: u16) -> Result<(), Error> {
        todo!()
    }

    fn i2c_read(&mut self, _: u8, _: u16) -> Result<(), Error> {
        todo!()
    }

    fn i2c_write(&mut self, _: u8, _: &[u16]) -> Result<(), Error> {
        todo!()
    }
}

impl Display for RaspiIo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read();
        write!(
            f,
            "{} [firmware={}, version={}, protocol={}]",
            self.get_name(),
            data.firmware_name,
            data.firmware_version,
            data.protocol_version
        )
    }
}
