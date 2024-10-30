use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use hermes_five::animations::Track;
use hermes_five::devices::Output;
use serde::{Deserialize, Serialize};

use crate::devices::device::DeviceType;
use crate::hardware::Board;
use crate::impl_device;

impl_device!(Led, {
    fn set_board(&mut self, board: &Board) -> Result<()> {
        let current = self.inner.clone();
        self.inner = hermes_five::devices::Led::new(
            &board.inner,
            current.get_pin(),
            current.get_default().as_bool(),
        )?;
        // @todo handle dimmable led.
        // .set_brightness(current.get_brightness())?;
        Ok(())
    }
});
