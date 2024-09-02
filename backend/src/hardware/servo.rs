use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use hermes_five::animation::Track;
use hermes_five::devices::Actuator;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;
use crate::impl_device;

impl_device!(Servo, {
    fn set_board(&mut self, board: &Board) -> anyhow::Result<()> {
        let current = self.inner.clone();
        self.inner = hermes_five::devices::Servo::new(
            &board.inner,
            current.get_pin(),
            current.get_default(),
        )?
        .set_type(current.get_type())
        .set_pwn_range(current.get_pwn_range())?
        .set_degree_range(current.get_degree_range())
        .set_range(current.get_range())
        .set_inverted(current.is_inverted());
        Ok(())
    }
});
