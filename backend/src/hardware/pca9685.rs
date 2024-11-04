use std::fmt::Debug;

use crate::impl_hardware;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PCA9685 {
    #[serde(flatten)]
    pub inner: hermes_five::hardware::PCA9685,
}

impl_hardware!(PCA9685);
