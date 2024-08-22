//! This file defines a structure called `Animation`.
//! @todo describe keyframes, etc...

use std::collections::HashMap;

use hermes_five::animation::Keyframe;
use serde::{Deserialize, Serialize};

use crate::impl_entity;
use crate::utils::entity::Entity;
use crate::utils::entity::Id;

/// Defines the structure of an animation entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    pub id: Id,
    /// The name of the animation.
    pub name: String,
    /// The description of the animation.
    pub description: String,
    /// Determines whether the segment should replay in a loop (starting from the [`Segment::loopback`] time).
    pub repeat: bool,
    /// The point in time (in ms) the animation will restart the loop when `loop` is set to true (default: 0).
    pub loopback: u64,
    /// Controls the speed of the animation in percent. (default: 100%)
    pub speed: u8,
    /// The number of frames per second (fps) for running the animation (default: 40fps).
    pub fps: u8,
    /// A hashmap of `Keyframe` keyed by `Group` id.
    pub keyframes: HashMap<Id, Vec<Keyframe>>,
}
impl_entity!(Animation);
