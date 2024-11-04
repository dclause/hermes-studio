use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    pub model: BoardType,
    #[serde(flatten)]
    pub inner: hermes_five::hardware::Board,
}

#[typetag::serde]
impl crate::hardware::HardwareTrait for Board {
    fn get_hardware(&mut self) -> &mut dyn hermes_five::hardware::Hardware {
        &mut self.inner
    }
}

// impl_hardware!(Board);

// ########################################

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum ArduinoType {
    NANO,
    UNO,
    MEGA,
    #[default]
    OTHER,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum RaspberryType {
    ZERO,
    #[allow(non_camel_case_types)]
    ZERO_W,
    TWO,
    THREE,
    FOUR,
    FIVE,
    #[default]
    OTHER,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub enum BoardType {
    Arduino(ArduinoType),
    RaspberryPi(RaspberryType),
    #[default]
    Unknown,
}
