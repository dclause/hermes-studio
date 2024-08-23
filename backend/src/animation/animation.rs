//! This file defines a structure called `Animation`.
//! @todo describe keyframes, etc...
use std::collections::HashMap;

use anyhow::Result;
use hermes_five::animation::Keyframe;
use serde::{Deserialize, Serialize};

use crate::animation::group::Group;
use crate::hardware::device::Device;
use crate::impl_entity;
use crate::utils::database::Database;
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

    // ########################################
    // # Volatile utility data.
    #[serde(skip)]
    pub inner: hermes_five::animation::Animation,
}
impl_entity!(Animation, {
    fn post_load(&mut self, database: &Database) -> Result<()> {
        self.build(database)?;
        Ok(())
    }
});

impl Animation {
    pub fn build(&mut self, database: &Database) -> Result<()> {
        let mut new_segment = hermes_five::animation::Segment::default()
            .set_repeat(self.repeat)
            .set_loopback(self.loopback)
            .set_speed(self.speed)
            .set_fps(self.fps);

        for (group_id, keyframes) in &self.keyframes {
            // Get the associated group.
            let group = database.get::<Group>(group_id)?;
            if let Some(group) = group {
                // Get the group device.
                if let Some(device_id) = group.device {
                    // Create a new track from the device: assign the keyframes and turn it into a segment.
                    let device = database.get::<Device>(&device_id)?.unwrap();
                    if let Some(mut new_track) = device.inner.into_track() {
                        for keyframe in keyframes {
                            new_track = new_track.with_keyframe(keyframe.clone());
                        }
                        new_segment = new_segment.with_track(new_track)
                    }
                }
            }
        }

        self.inner = hermes_five::animation::Animation::from(new_segment);
        Ok(())
    }
}
