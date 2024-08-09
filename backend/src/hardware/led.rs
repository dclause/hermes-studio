use std::fmt::Debug;

use anyhow::bail;
use hermes_five::devices::Actuator;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Led {
    #[serde(skip)]
    pub inner: Option<hermes_five::devices::Led>,

    // ########################################
    pub pin: u16,
    pub state: u16,
    pub default: u16,
    // ########################################
    intensity: u16,
    // ########################################
}

#[typetag::serde]
impl DeviceType for Led {
    fn init(&mut self, board: &Board) -> anyhow::Result<()> {
        self.inner = Some(hermes_five::devices::Led::new(&board.inner, self.pin)?);
        Ok(())
    }

    fn set_state(&mut self, state: u16) -> anyhow::Result<()> {
        match self.inner.as_mut() {
            None => bail!("Mutation on non initialized device"),
            Some(led) => led._set_state(state)?,
        }
        Ok(())
    }
}
