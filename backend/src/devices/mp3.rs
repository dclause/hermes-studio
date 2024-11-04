use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use hermes_five::animations::{Easing, Track};
use hermes_five::devices::Output;
use hermes_five::utils::State;

use crate::devices::DeviceType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mp3Player {
    #[serde(flatten)]
    pub inner: crate::extra::mp3::Mp3Player,
}

impl Deref for Mp3Player {
    type Target = crate::extra::mp3::Mp3Player;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Mp3Player {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[typetag::serde]
impl DeviceType for Mp3Player {
    fn reset(&mut self) -> Result<State> {
        let state = self.animate(self.inner.get_default(), 1000, Easing::SineInOut)?;
        Ok(state)
    }

    fn set_hardware(&mut self, hardware: &dyn hermes_five::hardware::Hardware) -> Result<()> {
        self.inner = crate::extra::mp3::Mp3Player::new(hardware)?.set_path(self.inner.get_path());
        Ok(())
    }

    fn set_state(&mut self, state: State) -> Result<State> {
        let state = self.inner.set_state(state.clone())?;
        Ok(state)
    }

    fn animate(&mut self, state: State, duration: u64, transition: Easing) -> Result<State> {
        self.inner.animate(state.clone(), duration, transition);
        Ok(state)
    }

    fn into_track(&self) -> Result<Track> {
        let device = self.inner.clone();
        Ok(Track::new(device))
    }
}
