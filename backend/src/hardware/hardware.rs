use crate::devices::Device;
use crate::impl_entity;
use crate::utils::database::{ArcDb, Database};
use crate::utils::entity::{Entity, Id};
use anyhow::Result;
use dyn_clone::DynClone;
use hermes_five::hardware::Hardware as HermesHardware;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hardware {
    pub id: Id,
    pub name: String,
    #[serde(flatten)]
    pub inner: Box<dyn HardwareTrait>,
    pub connected: bool,
}

impl_entity!(Hardware, {
    // Delete all associated devices.
    fn post_delete(&mut self, database: &mut Database) -> Result<()> {
        let devices = database.list::<Device>()?;
        for (_, device) in devices {
            if device.hid == self.id {
                database.delete::<Device>(device.id)?;
            }
        }
        Ok(())
    }
});

impl Hardware {
    pub fn open(mut self, database: &ArcDb) -> Result<Self> {
        let mut protocol = self.inner.get_hardware().get_protocol();
        protocol.open()?;
        self.connected = protocol.is_connected();
        self.inner.get_hardware().set_protocol(protocol);
        println!(
            "PROTOCOL CLONE: {}",
            self.inner.get_hardware().get_protocol()
        );
        //
        // protocol.set_pin_mode(13, PinModeId::OUTPUT)?;
        // protocol.digital_write(13, true)?;

        // let hardware = self.inner.get_hardware();
        // hardware.get_protocol().open()?;
        // // self.connected = protocol.is_connected();
        // println!("PROTOCOL: {}", hardware.get_protocol());
        //
        // protocol.set_pin_mode(13, PinModeId::OUTPUT)?;
        // protocol.digital_write(13, true)?;

        // self.inner.get_hardware().get_protocol().open()?;
        // self.connected = self.inner.get_hardware().is_connected();
        // println!("PROTOCOL: {}", self.inner.get_hardware().get_protocol());

        // self.inner
        //     .get_hardware()
        //     .get_protocol()
        //     .set_pin_mode(13, PinModeId::OUTPUT)?;
        // self.inner
        //     .get_hardware()
        //     .get_protocol()
        //     .digital_write(13, true)?;

        // Initialize properly the inner device value because now that Hardware is open(), the
        // handshake as given us the hardware configuration, which lets us properly initialize
        // our devices.
        let devices = database.write().list::<Device>()?;
        for (_, mut device) in devices {
            if device.hid == self.id {
                device.inner.set_hardware(self.inner.get_hardware())?;
                device.save(&database)?;
            }
        }

        Ok(self)
    }
    pub fn close(mut self) -> Result<Self> {
        self.inner.get_hardware().get_protocol().close()?;
        self.connected = false;
        Ok(self)
    }
}

#[typetag::serde(tag = "type")]
pub trait HardwareTrait: DynClone + Debug + Send + Sync {
    fn get_hardware(&mut self) -> &mut dyn HermesHardware;
}
dyn_clone::clone_trait_object!(HardwareTrait);

// ########################################
/// Helper macro to implement a [`Device`] for a given hermes_five device type.
#[macro_export]
macro_rules! impl_hardware {
    ($struct_name:ident $(, { $($additional_impl:item)* })?) => {
        #[typetag::serde]
        impl crate::hardware::HardwareTrait for $struct_name {
            fn get_hardware(&mut self) -> &mut dyn hermes_five::hardware::Hardware {
                &mut self.inner
            }
        }

        impl std::ops::Deref for $struct_name {
            type Target = hermes_five::hardware::$struct_name;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        // impl hermes_five::hardware::Hardware for $struct_name {
        //     fn get_protocol(&self) -> Box<dyn hermes_five::io::IoProtocol> {
        //         self.inner.get_protocol()
        //     }
        // }
        //
        // impl hermes_five::io::IO for $struct_name {
        //     fn get_io(&self) -> &std::sync::Arc<parking_lot::RwLock<hermes_five::io::IoData>> {
        //         self.inner.get_io()
        //     }
        //
        //     fn is_connected(&self) -> bool {
        //         self.inner.is_connected()
        //     }
        //
        //     fn set_pin_mode(
        //         &mut self,
        //         pin: u8,
        //         mode: hermes_five::io::PinModeId,
        //     ) -> Result<(), hermes_five::errors::Error> {
        //         self.inner.set_pin_mode(pin, mode)
        //     }
        //
        //     fn digital_write(
        //         &mut self,
        //         pin: u8,
        //         level: bool,
        //     ) -> Result<(), hermes_five::errors::Error> {
        //         self.inner.digital_write(pin, level)
        //     }
        //
        //     fn analog_write(
        //         &mut self,
        //         pin: u8,
        //         level: u16,
        //     ) -> Result<(), hermes_five::errors::Error> {
        //         self.inner.analog_write(pin, level)
        //     }
        //
        //     fn digital_read(&mut self, pin: u8) -> Result<bool, hermes_five::errors::Error> {
        //         self.inner.digital_read(pin)
        //     }
        //
        //     fn analog_read(&mut self, pin: u8) -> Result<u16, hermes_five::errors::Error> {
        //         self.inner.analog_read(pin)
        //     }
        //
        //     fn servo_config(
        //         &mut self,
        //         pin: u8,
        //         pwm_range: hermes_five::utils::Range<u16>,
        //     ) -> Result<(), hermes_five::errors::Error> {
        //         self.inner.servo_config(pin, pwm_range)
        //     }
        //
        //     fn i2c_config(&mut self, delay: u16) -> Result<(), hermes_five::errors::Error> {
        //         self.inner.i2c_config(delay)
        //     }
        //
        //     fn i2c_read(
        //         &mut self,
        //         address: u8,
        //         size: u16,
        //     ) -> Result<(), hermes_five::errors::Error> {
        //         self.inner.i2c_read(address, size)
        //     }
        //
        //     fn i2c_write(
        //         &mut self,
        //         address: u8,
        //         data: &[u16],
        //     ) -> Result<(), hermes_five::errors::Error> {
        //         self.inner.i2c_write(address, data)
        //     }
        // }
    };
}

// ########################################

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type")]
// pub enum HardwareType {
//     Board(Board),
//     PCA9685(PCA9685),
// }

// impl HardwareType {
//     pub fn get_hardware(&self) -> &dyn HermesHardware {
//         match &self {
//             HardwareType::Board(hardware) => &hardware.inner,
//             HardwareType::PCA9685(hardware) => &hardware.inner,
//         }
//     }
// }

// impl Deref for HardwareType {
//     type Target = dyn HermesHardware;
//
//     fn deref(&self) -> &Self::Target {
//         match self {
//             HardwareType::Board(ref hardware) => &hardware.inner,
//             HardwareType::PCA9685(ref hardware) => &hardware.inner,
//         }
//     }
// }
//
// impl DerefMut for HardwareType {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match self {
//             HardwareType::Board(ref mut hardware) => &mut hardware.inner,
//             HardwareType::PCA9685(ref mut hardware) => &mut hardware.inner,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::hardware::board::Board;
//     use crate::hardware::{Hardware, HardwareType};
//
//     #[test]
//     fn test_serialize() {
//         let hardware = Hardware {
//             id: 1,
//             name: "Hardware Test".to_string(),
//             inner: HardwareType::Board(Board {
//                 model: Default::default(),
//                 inner: Default::default(),
//             }),
//             connected: false,
//         };
//
//         let json = serde_json::to_string(&hardware).unwrap();
//         assert_eq!(
//             json,
//             r#"{"id":1,"name":"Hardware Test","type":"Board","model":"Unknown","protocol":{"type":"RemoteIo","transport":{"type":"Serial","port":"COM7"}},"connected":false}"#
//         );
//     }
//
//     #[test]
//     fn test_deserialize() {
//         let json = r#"{"id":1,"name":"Hardware Test","type":"Board","model":"Unknown","protocol":{"type":"RemoteIo","transport":{"type":"Serial","port":"COM7"}},"connected":false}"#;
//         let hardware = serde_json::from_str::<Hardware>(&json);
//         assert!(hardware.is_ok());
//         let hardware = hardware.unwrap();
//         assert_eq!(hardware.name, "Hardware Test".to_string());
//         assert_eq!(hardware.inner.get_protocol_name(), "RemoteIo".to_string());
//     }
// }
