use std::collections::HashMap;

use hermes_five::animation::Keyframe;
use serde::{Deserialize, Serialize};

use crate::animation::animation::Animation;
use crate::utils::entity::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnimationPayload {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub repeat: bool,
    pub loopback: u64,
    pub speed: u8,
    pub fps: u8,
    pub keyframes: HashMap<Id, Vec<Keyframe>>,
    pub playing: bool,
    pub duration: u64,
    pub progress: u64,
}

impl From<Animation> for AnimationPayload {
    fn from(animation: Animation) -> Self {
        Self {
            id: animation.id,
            name: animation.name,
            description: animation.description,
            repeat: animation.repeat,
            loopback: animation.loopback,
            speed: animation.speed,
            fps: animation.fps,
            playing: animation.inner.is_playing(),
            duration: animation.inner.get_duration(),
            progress: animation.inner.get_progress(),
            keyframes: animation.keyframes,
        }
    }
}
