//! This file defines a structure called `Animation`.
//! @todo describe keyframes, etc...
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use hermes_five::animation::Track;
use hermes_five::utils::task;
use hermes_five::utils::task::TaskHandler;
use parking_lot::RwLock;
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
    pub tracks: HashMap<Id, Vec<Keyframe>>,

    // ########################################
    // # Volatile utility data.
    #[serde(skip)]
    pub inner: hermes_five::animation::Animation,
    #[serde(skip)]
    pub interval: Arc<RwLock<Option<TaskHandler>>>,
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

        let mut tracks: HashMap<Id, Track> = HashMap::new();
        // Loop through each tracks of the animation (one track per group)
        for (group_id, keyframes) in &self.tracks {
            // Retrieve the device ID associated with the group, if any
            let device_id = match database.get::<Group>(group_id).unwrap() {
                None => continue, // Skip if the group no longer exists
                Some(group) => group.device,
            };

            // Fetch the device associated with the device_id, if any
            let (mut device_id, mut track) = match device_id {
                None => (0, None),
                Some(device_id) => match database.get::<Device>(&device_id).unwrap() {
                    None => continue, // Skip if the group no longer exists
                    Some(device) => match device.inner.clone().into_track() {
                        Err(_) => continue, // Skip if device is not applicable as a track (device is not Actuator for instance)
                        Ok(track) => (device_id, Some(track)),
                    },
                },
            };

            for keyframe in keyframes {
                // If we are not supposed to have a track already, then we can proceed to:
                if device_id == 0 {
                    track = match tracks.get(&keyframe.device).cloned().or_else(|| {
                        database
                            .get::<Device>(&keyframe.device)
                            .unwrap()
                            .and_then(|device| device.inner.into_track().ok())
                    }) {
                        None => continue,
                        Some(track) => Some(track),
                    };
                    device_id = keyframe.device;
                };

                if let Some(_track) = track {
                    track = Some(_track.with_keyframe(keyframe.clone().into()));
                }
            }

            if keyframes.len() > 0 && track.is_some() && device_id > 0 {
                tracks.insert(device_id, track.unwrap());
            }
        }

        for (_, track) in tracks {
            new_segment = new_segment.with_track(track)
        }

        self.inner = hermes_five::animation::Animation::from(new_segment);
        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        let mut self_clone = self.clone();
        let handler = task::run(async move {
            // Loop through the segments and run them one by one.
            for index in self_clone.inner.get_current()..self_clone.inner.get_segments().len() {
                self_clone.inner.set_current(index);

                // Retrieve the currently running segment.
                let segment_playing = self_clone.inner.get_mut_segments().get_mut(index).unwrap();
                segment_playing.play()?;
            }

            self_clone.inner.set_current(0); // reset to the beginning
            Ok(())
        })?;
        *self.interval.write() = Some(handler);

        Ok(())
    }

    // pub fn play(&mut self) -> Result<()> {
    //     // Loop through the segments and run them one by one.
    //     for index in self.inner.get_current()..self.inner.get_segments().len() {
    //         self.inner.set_current(index);
    //
    //         // Retrieve the currently running segment.
    //         let segment_playing = self.inner.get_mut_segments().get_mut(index).unwrap();
    //         segment_playing.play()?;
    //     }
    //
    //     self.inner.set_current(0); // reset to the beginning
    //
    //     Ok(())
    // }
}

// ######################################

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyframe {
    #[serde(flatten)]
    inner: hermes_five::animation::Keyframe,
    device: Id,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    position: Vec<Position>,
}

impl Into<hermes_five::animation::Keyframe> for Keyframe {
    fn into(self) -> hermes_five::animation::Keyframe {
        self.inner
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    device: Id,
    target: u16,
}
