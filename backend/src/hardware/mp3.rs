use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use anyhow::Result;
use hermes_five::animation::Track;
use hermes_five::devices::Actuator;
use hermes_five::utils::State;
use serde::{Deserialize, Serialize};

use crate::hardware::board::Board;
use crate::hardware::device::DeviceType;

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
    fn set_board(&mut self, board: &Board) -> Result<()> {
        let current = self.inner.clone();
        self.inner = crate::extra::mp3::Mp3Player::new(&board.inner)?.set_path(current.get_path());
        Ok(())
    }

    fn animate(
        &mut self,
        state: State,
        duration: u64,
        transition: hermes_five::utils::Easing,
    ) -> Result<State> {
        self.inner.animate(state.clone(), duration, transition);
        Ok(state)
    }

    fn set_state(&mut self, state: State) -> Result<State> {
        let state = self.inner.set_state(state.clone())?;
        Ok(state)
    }

    fn into_track(&self) -> Result<Track> {
        let device = self.inner.clone();
        Ok(Track::new(device))
    }

    fn reset(&mut self) -> Result<State> {
        let state = self.animate(
            self.inner.get_default(),
            1000,
            hermes_five::utils::Easing::SineInOut,
        )?;
        Ok(state)
    }
}
