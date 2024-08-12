use std::fmt::Debug;

use anyhow::bail;
use hermes_five::devices::Actuator;
use hermes_five::utils::Range;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Servo {
    #[serde(skip)]
    pub inner: Option<hermes_five::devices::Servo>,

    // ########################################
    pub pin: u16,
    pub state: u16,
    pub default: u16,
    // ########################################
    servo_type: hermes_five::devices::ServoType,
    range: Range<u16>,
    pwm_range: Range<u16>,
    degree_range: Range<u16>,
    // ########################################
}

#[typetag::serde]
impl DeviceType for Servo {
    fn init(&mut self, board: &Board) -> anyhow::Result<()> {
        self.inner = Some(hermes_five::devices::Servo::new(
            &board.inner,
            self.pin,
            self.default,
        )?);
        Ok(())
    }

    fn set_state(&mut self, state: u16) -> anyhow::Result<u16> {
        match self.inner.as_mut() {
            None => bail!("Mutation on non initialized device"),
            Some(servo) => servo.set_state(state)?,
        };
        Ok(state)
    }
}
