use std::fmt::{Display, Formatter};
use std::sync::Arc;

use hermes_five::errors::Error;
use hermes_five::io::{IoData, IoProtocol, PinModeId};
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
    fn get_data(&self) -> &Arc<RwLock<IoData>> {
        &self.data
    }
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
    fn is_connected(&self) -> bool {
        self.connected
    }
    fn set_pin_mode(&mut self, pin: u16, mode: PinModeId) -> Result<(), Error> {
        todo!()
    }
    fn digital_write(&mut self, pin: u16, level: bool) -> Result<(), Error> {
        todo!()
    }
    fn analog_write(&mut self, pin: u16, level: u16) -> Result<(), Error> {
        todo!()
    }
    fn report_analog(&mut self, channel: u8, state: bool) -> Result<(), Error> {
        todo!()
    }
    fn report_digital(&mut self, pin: u16, state: bool) -> Result<(), Error> {
        todo!()
    }
    fn sampling_interval(&mut self, interval: u16) -> Result<(), Error> {
        todo!()
    }
    fn i2c_config(&mut self, delay: u16) -> Result<(), Error> {
        todo!()
    }
    fn i2c_read(&mut self, address: i32, size: i32) -> Result<(), Error> {
        todo!()
    }
    fn i2c_write(&mut self, address: i32, data: &[u8]) -> Result<(), Error> {
        todo!()
    }
    fn servo_config(&mut self, pin: u16, pwm_range: Range<u16>) -> Result<(), Error> {
        todo!()
    }
}

impl Display for RaspiIo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read();
        write!(
            f,
            "{} [firmware={}, version={}, protocol={}]",
            self.get_protocol_name(),
            data.firmware_name,
            data.firmware_version,
            data.protocol_version
        )
    }
}
