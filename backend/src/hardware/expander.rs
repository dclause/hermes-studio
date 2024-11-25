use crate::devices::Device;
use crate::hardware::HardwareType;
use crate::impl_entity;
use crate::utils::database::{ArcDb, Database};
use crate::utils::entity::{Entity, Id};
use anyhow::Result;
use dyn_clone::DynClone;
use hermes_five::hardware::Hardware as HermesHardware;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expander {
    pub id: Id,
    pub hid: HardwareType,
    pub name: String,
    #[serde(flatten)]
    pub inner: Box<dyn ExpanderTrait>,
    #[serde(skip)]
    pub connected: bool,
}

impl_entity!(Expander, {
    // Delete all associated devices.
    fn post_delete(&mut self, database: &mut Database) -> Result<()> {
        let devices = database.list::<Device>()?;
        for (_, device) in devices {
            match device.hid {
                HardwareType::Board(_) => {}
                HardwareType::Expander(id) => {
                    if id == self.id {
                        database.delete::<Device>(device.id)?;
                    }
                }
            }
        }
        Ok(())
    }
});

impl Expander {
    pub fn open(mut self, database: &ArcDb) -> Result<Self> {
        let mut protocol = self.inner.get_hardware().get_protocol();
        protocol.open()?;
        self.inner.get_mut_hardware().set_protocol(protocol);
        self.connected = true;

        // Initialize properly the inner expander value because now that board is open(), the
        // handshake as given us the hardware board configuration, which lets us properly initialize
        // our expanders.
        let devices = database.write().list::<Device>()?;
        for (_, mut device) in devices {
            match device.hid {
                HardwareType::Board(_) => {}
                HardwareType::Expander(id) => {
                    println!("Set expander for device {}", device.name);
                    if id == self.id {
                        device.inner.set_hardware(self.inner.get_hardware())?;
                        device.save(&database)?;
                    }
                }
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
pub trait ExpanderTrait: DynClone + Debug + Send + Sync {
    fn get_hardware(&self) -> &dyn HermesHardware;
    fn get_mut_hardware(&mut self) -> &mut dyn HermesHardware;
    fn set_hardware(&mut self, hardware: &dyn hermes_five::hardware::Hardware) -> Result<()>;
}
dyn_clone::clone_trait_object!(ExpanderTrait);

// ########################################
/// Helper macro to implement a [`Device`] for a given hermes_five device type.
#[macro_export]
macro_rules! impl_expander {
    ($struct_name:ident $(, { $($additional_impl:item)* })?) => {
        #[typetag::serde]
        impl crate::hardware::ExpanderTrait for $struct_name {
            fn get_hardware(&self) -> &dyn hermes_five::hardware::Hardware {
                &self.inner
            }
            fn get_mut_hardware(&mut self) -> &mut dyn hermes_five::hardware::Hardware {
                &mut self.inner
            }

            // Apply additional methods if provided
            $(
                $($additional_impl)*
            )?
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

        // impl hermes_five::hardware::Expander for $struct_name {
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
// pub enum ExpanderType {
//     Board(Board),
//     PCA9685(PCA9685),
// }

// impl ExpanderType {
//     pub fn get_hardware(&self) -> &dyn HermesExpander {
//         match &self {
//             ExpanderType::Board(hardware) => &hardware.inner,
//             ExpanderType::PCA9685(hardware) => &hardware.inner,
//         }
//     }
// }

// impl Deref for ExpanderType {
//     type Target = dyn HermesExpander;
//
//     fn deref(&self) -> &Self::Target {
//         match self {
//             ExpanderType::Board(ref hardware) => &hardware.inner,
//             ExpanderType::PCA9685(ref hardware) => &hardware.inner,
//         }
//     }
// }
//
// impl DerefMut for ExpanderType {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         match self {
//             ExpanderType::Board(ref mut hardware) => &mut hardware.inner,
//             ExpanderType::PCA9685(ref mut hardware) => &mut hardware.inner,
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::hardware::pca9685::PCA9685;
    use crate::hardware::Expander;
    use hermes_five::mocks::plugin_io::MockIoProtocol;

    #[test]
    fn test_serialize() {
        let board = hermes_five::hardware::Board::new(MockIoProtocol::default());
        let hardware = Expander {
            id: 1,
            name: "Expander Test".to_string(),
            hid: 1,
            inner: Box::new(PCA9685 {
                inner: hermes_five::hardware::PCA9685::new(&board, 0x66).unwrap(),
            }),
            connected: true,
        };

        let json = serde_json::to_string(&hardware).unwrap();
        assert_eq!(
            json,
            r#"{"id":1,"hid":1,"name":"Expander Test","type":"PCA9685","address":102,"frequency":50,"connected":true}"#
        );
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"id":1,"hid":1,"name":"Expander Test","type":"PCA9685","address":102,"frequency":50,"connected":true}"#;
        let expander = serde_json::from_str::<Expander>(&json);
        assert!(expander.is_ok());
        let expander = expander.unwrap();
        assert_eq!(expander.name, "Expander Test".to_string());
        assert!(expander.inner.get_hardware().is_connected());
        assert_eq!(expander.inner.get_hardware().get_protocol_name(), "PCA9685");
    }
}
