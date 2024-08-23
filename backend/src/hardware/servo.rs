use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use hermes_five::animation::Track;
use hermes_five::devices::Actuator;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;
use crate::impl_device;

impl_device!(Servo, {
    fn reset(&mut self, board: &Board) -> anyhow::Result<()> {
        let current = self.inner.clone();
        self.inner = hermes_five::devices::Servo::new(
            &board.inner,
            current.get_pin(),
            current.get_default(),
        )?
        .set_type(current.get_type())
        .set_pwn_range(current.get_pwn_range())?
        .set_degree_range(current.get_degree_range())
        .set_range(current.get_range());
        Ok(())
    }

    fn into_track(&self) -> Option<Track> {
        let device = self.inner.clone();
        Some(Track::new(device))
    }
});
