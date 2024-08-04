use serde::Deserialize;

use crate::hardware::board::Board;

#[derive(Deserialize, Debug)]
pub struct CreateBoard {
    pub(crate) name: String,
}

impl Into<Board> for CreateBoard {
    fn into(self) -> Board {
        Board {
            id: 0,
            name: self.name,
            inner: Default::default(),
        }
    }
}
