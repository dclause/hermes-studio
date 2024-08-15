use serde::Deserialize;

use crate::hardware::board::{Board, BoardType};

// ########################################
// API data exchange.

#[derive(Deserialize, Debug)]
pub struct CreateBoard {
    pub name: String,
    #[allow(dead_code)]
    pub model: BoardType,
}

impl Into<Board> for CreateBoard {
    fn into(self) -> Board {
        Board {
            id: 0,
            name: self.name,
            inner: Default::default(),
            connected: false,
            model: Default::default(),
        }
    }
}
