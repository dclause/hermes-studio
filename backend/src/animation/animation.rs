//! This file defines a structure called `Animation`.
//! @todo describe keyframes, etc...
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use hermes_five::animation::Track;
use hermes_five::utils::{Easing, task};
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
            // First: ensure the group associated with the track still exists: if not abort this track.
            if database.get::<Group>(group_id)?.is_none() {
                continue;
            };

            // Loop over the track keyframes...
            for keyframe in keyframes {
                // But for "group keyframes" purpose (see frontend), we harmonised device-track
                // (track which group is actually a device) and group-track (tracks for group where a keyframes
                // will control multiple devices) so each keyframe contains positions (can be an array of one)
                // that we need to loop through.
                for position in &keyframe.positions {
                    // 1. ensure the device associated with the position still exists: if not abort this track.
                    let device = match database.get::<Device>(&position.device)? {
                        None => continue,
                        Some(device) => device,
                    };

                    // 2. Retrieve the hermes-track for the device (if already created) or create a new one
                    // for the current frontend-track.
                    let track = match tracks.get(&position.device) {
                        Some(track) => track.clone(),
                        None => device.inner.into_track()?,
                    };

                    // 3. Add the position as a new hermes-keyframe on the hermes-track.
                    let track = track.with_keyframe(
                        hermes_five::animation::Keyframe::new(
                            position.target,
                            keyframe.start,
                            keyframe.end,
                        )
                        .set_transition(keyframe.transition),
                    );

                    tracks.insert(device.id, track);
                }
            }
        }

        // Add the tracks to the new animation segment.
        for (_, track) in tracks {
            new_segment = new_segment.with_track(track)
        }

        self.inner = hermes_five::animation::Animation::from(new_segment);
        println!("{}", self.inner);
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
    positions: Vec<Position>,
    start: u64,
    end: u64,
    transition: Easing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    device: Id,
    target: u16,
}
