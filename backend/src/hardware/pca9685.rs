use crate::impl_expander;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PCA9685 {
    #[serde(flatten)]
    pub inner: hermes_five::hardware::PCA9685,
}

impl_expander!(PCA9685, {
    fn set_hardware(
        &mut self,
        hardware: &dyn hermes_five::hardware::Hardware,
    ) -> anyhow::Result<()> {
        self.inner = hermes_five::hardware::PCA9685::new(hardware, self.inner.get_address())?;
        Ok(())
    }
});

#[cfg(test)]
mod tests {
    use crate::hardware::pca9685::PCA9685;
    use hermes_five::hardware::Hardware;
    use hermes_five::io::{IoProtocol, IO};
    use hermes_five::mocks::plugin_io::MockIoProtocol;

    #[test]
    fn test_serialize() {
        let board = hermes_five::hardware::Board::new(MockIoProtocol::default());
        let pca9685 = PCA9685 {
            inner: hermes_five::hardware::PCA9685::default(&board).unwrap(),
        };

        let json = serde_json::to_string(&pca9685).unwrap();
        assert_eq!(json, r#"{"address":64,"frequency":50,"connected":true}"#);
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{"address":64,"frequency":50,"connected":true}"#;
        let pca9685 = serde_json::from_str::<PCA9685>(&json);
        assert!(pca9685.is_ok());
        let pca9685 = pca9685.unwrap();
        assert_eq!(pca9685.inner.get_protocol_name(), "PCA9685".to_string());
        assert_eq!(pca9685.inner.get_name(), "PCA9685".to_string());
        assert!(pca9685.inner.is_connected());
    }
}
