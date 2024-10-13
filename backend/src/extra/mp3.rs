use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::sync::Arc;

use anyhow::Result;
use hermes_five::{Board, pause_sync};
use hermes_five::devices::{Actuator, Device};
use hermes_five::errors::Error;
use hermes_five::utils::{Easing, State};
use hermes_five::utils::events::{EventHandler, EventManager};
use parking_lot::RwLock;
use rodio::{Decoder, OutputStream, Sink, Source};
use serde::{Deserialize, Serialize};

/// Lists all events an Mp3Player device can emit/listen.
pub enum Mp3PlayerEvent {
    /// Triggered when the song starts.
    OnStart,
    /// Triggered when the song stops.
    OnEnd,
}

/// Convert events to string to facilitate usage with [`EventManager`].
impl Into<String> for Mp3PlayerEvent {
    fn into(self) -> String {
        let event = match self {
            Mp3PlayerEvent::OnStart => "start",
            Mp3PlayerEvent::OnEnd => "end",
        };
        event.into()
    }
}

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

#[derive(Clone, Serialize, Deserialize)]
pub struct Mp3Player {
    /// The current Mp3Player state.
    #[serde(with = "hermes_five::devices::arc_rwlock_serde")]
    state: Arc<RwLock<Mp3PlayerState>>,
    /// The Mp3Player default value.
    default: Mp3PlayerState,

    /// The mp3 controller (play/pause/stop)
    #[serde(skip)]
    control: Arc<RwLock<Option<Sink>>>,
    /// The event manager for the animation.
    #[serde(skip)]
    events: EventManager,
}

impl Mp3Player {
    pub fn new(_board: &Board) -> Result<Self, Error> {
        Ok(Self {
            state: Arc::new(Default::default()),
            default: Default::default(),
            control: Arc::new(RwLock::new(None)),
            events: Default::default(),
        })
    }

    /// Changes the current song (and play it if currently playing).
    pub fn change_track<P: Into<String>>(&mut self, path: P) -> Result<(), Error> {
        self.stop();
        self.state.write().path = path.into();
        self.play(u64::MAX)
    }

    /// Plays the current song.
    pub fn play(&mut self, duration: u64) -> Result<(), Error> {
        // Resume control if the song is not done.
        if let Some(control) = self.control.read().as_ref() {
            if !control.empty() {
                control.play();
            }
        } else if !self.state.read().path.is_empty() {
            let self_clone = self.clone();

            tokio::task::spawn_blocking(move || {
                // Start playing the current file
                // Get an output stream handle to the default physical sound device
                let (_stream, stream_handle) =
                    OutputStream::try_default().map_err(|err| hermes_five::errors::Unknown {
                        info: err.to_string(),
                    })?;

                // Load a sound from a file, using a path relative to Cargo.toml
                let path = self_clone.state.read().path.clone();
                let file = BufReader::new(File::open(path)?);

                // Decode that sound file into a source
                let source = Decoder::new(file)
                    .map_err(|err| hermes_five::errors::Unknown {
                        info: err.to_string(),
                    })?
                    .convert_samples::<f32>();
                let duration = source
                    .total_duration()
                    .unwrap()
                    .as_millis()
                    .min(duration as u128);

                // Play the sound directly on the device
                let sink =
                    Sink::try_new(&stream_handle).map_err(|err| hermes_five::errors::Unknown {
                        info: err.to_string(),
                    })?;
                sink.append(source);

                self_clone
                    .events
                    .emit(Mp3PlayerEvent::OnStart, self_clone.clone());

                // Save the sink for later control.
                *self_clone.control.write() = Some(sink);

                pause_sync!(duration);

                // Delete the sink now that it is not needed anymore.
                *self_clone.control.write() = None;
                self_clone
                    .events
                    .emit(Mp3PlayerEvent::OnEnd, self_clone.clone());

                Ok::<(), Error>(())
            });
        }

        self.state.write().status = Mp3Command::PLAY;
        Ok(())
    }

    /// Plays the current song.
    pub fn pause(&mut self) {
        match self.control.read().deref() {
            None => {}
            Some(control) => control.pause(),
        };
        self.state.write().status = Mp3Command::PAUSE;
    }

    pub fn get_path(&self) -> String {
        self.state.read().path.clone()
    }

    /// Set the path (this does not change the current playing file).
    pub fn set_path<P: Into<String>>(self, path: P) -> Self {
        self.state.write().path = path.into();
        self
    }

    // ########################################
    // Event related functions

    /// Registers a callback to be executed on a given event. To use it, register though the [`Self::on()`] method.
    ///
    /// Available events are:
    /// * `OnStart` | `start`: Triggered when the song starts. To use it, register though the [`Self::on()`] method.
    /// * `OnEnd` | `end`: Triggered when the song ends. To use it, register though the [`Self::on()`] method.
    /// ```
    pub fn on<S, F, T, Fut>(&self, event: S, callback: F) -> EventHandler
    where
        S: Into<String>,
        T: 'static + Send + Sync + Clone,
        F: FnMut(T) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = std::result::Result<(), Error>> + Send + 'static,
    {
        self.events.on(event, callback)
    }
}

#[typetag::serde]
impl Device for Mp3Player {}

#[typetag::serde]
impl Actuator for Mp3Player {
    fn animate<S: Into<State>>(&mut self, state: S, duration: u64, _transition: Easing)
    where
        Self: Sized,
    {
        match state.into().clone() {
            State::Object(state) => {
                if let Some(path) = state.get("path") {
                    self.state.write().path = path.as_string();
                }
                let _ = self.play(duration);
            }
            state => {
                let _ = self.set_state(state.clone());
            }
        }
    }

    /// Stops the current song.
    fn stop(&mut self) {
        // match self.control.read().deref() {
        //     None => {}
        //     Some(control) => control.stop(),
        // };
        *self.control.write() = None;
        self.state.write().status = Mp3Command::STOP;
    }

    fn set_state(&mut self, state: State) -> Result<State, Error> {
        match state.clone() {
            State::Null => {}
            State::Boolean(_) => {}
            State::Integer(command) => match Mp3Command::from(command as i8) {
                Mp3Command::PLAY => self.play(u64::MAX)?,
                Mp3Command::PAUSE => self.pause(),
                Mp3Command::STOP => self.stop(),
            },
            State::Signed(command) => match Mp3Command::from(command as i8) {
                Mp3Command::PLAY => self.play(u64::MAX)?,
                Mp3Command::PAUSE => self.pause(),
                Mp3Command::STOP => self.stop(),
            },
            State::Float(_) => {}
            State::String(path) => self.state.write().path = path,
            State::Array(_) => {}
            State::Object(state) => {
                if let Some(path) = state.get("path") {
                    self.state.write().path = path.as_string();
                }
                if let Some(status) = state.get("status") {
                    let _ = self.set_state(status.clone());
                }
            }
        };
        let new_state = self.state.read().clone();
        Ok(State::into_state(new_state))
    }

    fn get_state(&self) -> State {
        State::into_state(self.state.read().clone())
    }

    fn get_default(&self) -> State {
        State::into_state(self.default.clone())
    }

    fn is_busy(&self) -> bool {
        match self.control.read().as_ref() {
            None => false,
            Some(control) => !control.empty(),
        }
    }

    fn scale_state(&mut self, _previous: State, target: State, progress: f32) -> State {
        match progress > 0.01 {
            true => State::Null,
            _ => target,
        }
    }
}

impl Display for Mp3Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mp3Player [path={}]", self.state.read().path)
    }
}

impl Debug for Mp3Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mp3Player")
            .field("state", &self.state)
            .field("default", &self.default)
            .field("protocol", &"[Box<dyn Protocol>]")
            .field("controls", &"[audio control]")
            .finish()
    }
}
