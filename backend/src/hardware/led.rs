use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use hermes_five::animation::Track;
use hermes_five::devices::Actuator;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;
use crate::impl_device;

impl_device!(Led, {
    fn set_board(&mut self, board: &Board) -> Result<()> {
        let current = self.inner.clone();
        self.inner = hermes_five::devices::Led::new(&board.inner, current.get_pin())?
            .set_intensity(current.get_intensity())?;
        Ok(())
    }
});
