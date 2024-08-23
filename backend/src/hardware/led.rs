use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use hermes_five::animation::Track;
use hermes_five::devices::Actuator;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;
use crate::impl_device;

impl_device!(Led, {
    fn reset(&mut self, board: &Board) -> anyhow::Result<()> {
        let current = self.inner.clone();
        self.inner = hermes_five::devices::Led::new(&board.inner, current.get_pin())?
            .set_intensity(current.get_intensity())?;
        Ok(())
    }

    fn into_track(&self) -> Option<Track> {
        let device = self.inner.clone();
        Some(Track::new(device))
    }
});
