use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use hermes_five::devices::Actuator;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;
use crate::impl_device;

impl_device!(Servo, {
    fn reset(&mut self, board: &Board) -> anyhow::Result<()> {
        self.inner =
            hermes_five::devices::Servo::new(&board.inner, self.get_pin(), self.get_default())?;
        Ok(())
    }
});
