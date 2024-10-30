use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use hermes_five::animations::Track;
use hermes_five::devices::Output;
use serde::{Deserialize, Serialize};

use crate::devices::DeviceType;
use crate::hardware::Board;
use crate::impl_device;

impl_device!(Servo, {
    fn set_board(&mut self, board: &Board) -> Result<()> {
        let current = self.inner.clone();

        self.inner = match current.is_inverted() {
            false => hermes_five::devices::Servo::new(
                &board.inner,
                current.get_pin(),
                current.get_default().as_integer() as u16,
            )?,
            true => hermes_five::devices::Servo::new_inverted(
                &board.inner,
                current.get_pin(),
                (100 - current.get_default().as_integer()) as u16,
            )?,
        }
        .set_type(current.get_type())
        .set_pwn_range(current.get_pwn_range())?
        .set_degree_range(current.get_degree_range())
        .set_range(current.get_range())
        .set_auto_detach(current.is_auto_detach())
        .set_detach_delay(current.get_detach_delay());

        Ok(())
    }
});
