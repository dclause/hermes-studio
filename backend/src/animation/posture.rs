use serde::{Deserialize, Serialize};

use crate::animation::animation::Position;
use crate::hardware::board::Board;
use crate::hardware::device::Device;
use crate::impl_entity;
use crate::utils::database::Database;
use crate::utils::entity::Id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Posture {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub positions: Vec<Position>,
}

impl_entity!(Posture);

impl Posture {
    pub fn play(&mut self, database: &Database) -> anyhow::Result<()> {
        for position in &self.positions {
            database
                .get::<Device>(&position.device)
                .and_then(|device| match device {
                    None => Ok(()), // Do not bother with unknown devices
                    Some(mut device) => {
                        database
                            .get::<Board>(&device.bid)
                            .and_then(|board| match board {
                                None => Ok(()), // Should not happen ?
                                Some(board) => match board.connected {
                                    false => Ok(()), // Do not bother with none connected boards.
                                    true => {
                                        device.inner.set_state(position.target.clone()).map(|_| ())
                                    }
                                },
                            })
                    }
                })?;
        }
        Ok(())
    }
}
