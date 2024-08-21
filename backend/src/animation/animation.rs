//! This file defines a structure called `Animation`.
//! @todo describe keyframes, etc...

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
}

impl_entity!(Animation);
