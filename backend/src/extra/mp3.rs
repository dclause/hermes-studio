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

#[repr(i8)]
#[derive(Clone, Debug, Default)]
enum Mp3Command {
    PLAY = 1,
    PAUSE = 0,
    #[default]
    STOP = -1,
}

impl From<i8> for Mp3Command {
    fn from(value: i8) -> Self {
        if value < 0 {
            Mp3Command::STOP
        } else if value > 0 {
            Mp3Command::PLAY
        } else {
            Mp3Command::PAUSE
        }
    }
}

impl<'de> Deserialize<'de> for Mp3Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Mp3Command::from(i8::deserialize(deserializer)?))
    }
}
impl Serialize for Mp3Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8(self.clone() as i8)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct Mp3PlayerState {
    path: String,
    status: Mp3Command,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mp3Player {
    /// The current Mp3Player state.
    #[serde(with = "hermes_five::devices::arc_rwlock_serde")]
    state: Arc<RwLock<Mp3PlayerState>>,
    /// The Mp3Player default value.
    default: u16,

    #[serde(skip)]
    protocol: Box<dyn Protocol>,
    /// Inner handler to the task running the animation.
    #[serde(skip)]
    interval: Arc<Option<TaskHandler>>,
    /// Inner handler to the task running the animation.
    #[serde(skip)]
    animation: Arc<RwLock<Option<Animation>>>,
}

impl Mp3Player {
    pub fn new(board: &Board) -> Result<Self, Error> {
        let protocol = board.get_protocol();
        Ok(Self {
            state: Arc::new(Default::default()),
            default: 0,
            protocol,
            interval: Arc::new(None),
            animation: Arc::new(RwLock::new(None)),
        })
    }

    /// Changes the current song (and play it if currently playing).
    pub fn change_music<P: Into<String>>(&mut self, path: P) {
        self.state.write().path = path.into();
    }

    /// Plays the current song.
    pub fn play(&mut self) {
        self.state.write().status = Mp3Command::PLAY;
    }

    /// Plays the current song.
    pub fn pause(&mut self) {
        self.state.write().status = Mp3Command::PAUSE;
    }

    /// Plays the current song.
    pub fn stop(&mut self) {
        self.state.write().status = Mp3Command::STOP;
    }

    pub fn get_path(&self) -> String {
        self.state.read().path.clone()
    }

    /// Set the path (this does not changes the current playing file).
    pub fn set_path<P: Into<String>>(mut self, path: P) -> Self {
        self.state.write().path = path.into();
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
        match state.clone() {
            State::Null => {}
            State::Boolean(_) => {}
            State::Integer(command) => match Mp3Command::from(command as i8) {
                Mp3Command::PLAY => self.play(),
                Mp3Command::PAUSE => self.pause(),
                Mp3Command::STOP => self.stop(),
            },
            State::Signed(command) => match Mp3Command::from(command as i8) {
                Mp3Command::PLAY => self.play(),
                Mp3Command::PAUSE => self.pause(),
                Mp3Command::STOP => self.stop(),
            },
            State::Float(_) => {}
            State::String(path) => self.state.write().path = path,
            State::Array(_) => {}
            State::Object(_) => {}
        };
        let new_state = self.state.read().clone();
        Ok(State::into_state(new_state))
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
