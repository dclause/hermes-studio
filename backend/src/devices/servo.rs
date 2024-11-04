use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::devices::device::DeviceType;
use crate::impl_device;
use hermes_five::animations::Track;
use hermes_five::devices::Output;

impl_device!(Servo, {
    fn set_hardware(&mut self, hardware: &dyn hermes_five::hardware::Hardware) -> Result<()> {
        let current = self.inner.clone();
        self.inner = match current.is_inverted() {
            false => hermes_five::devices::Servo::new(
                hardware,
                current.get_pin(),
                current.get_default().as_integer() as u16,
            )?,
            true => hermes_five::devices::Servo::new_inverted(
                hardware,
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
