use std::fmt::{Display, Formatter};
use std::sync::Arc;

use hermes_five::animation::Animation;
use hermes_five::Board;
use hermes_five::devices::{Actuator, Device};
use hermes_five::errors::Error;
use hermes_five::protocols::Protocol;
use hermes_five::utils::{Easing, State};
use hermes_five::utils::task::TaskHandler;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mp3Player {
    /// The current Mp3Player state.
    #[serde(with = "hermes_five::devices::arc_rwlock_serde")]
    state: Arc<RwLock<u16>>,
    /// The Mp3Player default value.
    default: u16,
    /// The Mp3 file path
    path: String,

    #[serde(skip)]
    protocol: Box<dyn Protocol>,
    /// Inner handler to the task running the animation.
    #[serde(skip)]
    interval: Arc<Option<TaskHandler>>,
    /// Inner handler to the task running the animation.
    #[serde(skip)]
    animation: Arc<Option<Animation>>,
}

impl Mp3Player {
    pub fn new(board: &Board) -> Result<Self, Error> {
        let protocol = board.get_protocol();
        Ok(Self {
            state: Arc::new(Default::default()),
            default: 0,
            path: String::default(),
            protocol,
            interval: Arc::new(None),
            animation: Arc::new(None),
        })
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn set_path<P: Into<String>>(mut self, path: P) -> Self {
        self.path = path.into();
        self
    }
}

#[typetag::serde]
impl Device for Mp3Player {}

#[typetag::serde]
impl Actuator for Mp3Player {
    fn animate<S: Into<State>>(&mut self, state: S, duration: u64, transition: Easing)
    where
        Self: Sized,
    {
        todo!()
    }

    fn set_state(&mut self, state: State) -> Result<State, Error> {
        todo!()
    }

    fn get_state(&self) -> State {
        todo!()
    }

    fn get_default(&self) -> State {
        todo!()
    }

    fn is_busy(&self) -> bool {
        todo!()
    }
}

impl Display for Mp3Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
