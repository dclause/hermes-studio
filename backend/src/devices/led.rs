use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use hermes_five::animations::Track;
use hermes_five::devices::Output;

use crate::devices::device::DeviceType;
use crate::impl_device;

impl_device!(Led, {
    fn set_hardware(&mut self, hardware: &dyn hermes_five::hardware::Hardware) -> Result<()> {
        self.inner = hermes_five::devices::Led::new(
            hardware,
            self.inner.get_pin(),
            self.inner.get_default().as_bool(),
        )?;
        Ok(())
    }
});
