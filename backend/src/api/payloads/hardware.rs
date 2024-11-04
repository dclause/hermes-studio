use serde::{Deserialize, Serialize};

use crate::hardware::{Board, BoardType};
use crate::utils::entity::Id;
use hermes_five::hardware::Board as HermesBoard;

// ########################################
// API data exchange.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoardPayload {
    pub id: Id,
    pub name: String,
    pub model: BoardType,
    pub connected: bool,
    #[serde(flatten)]
    pub inner: HermesBoard,
}

impl From<Board> for BoardPayload {
    fn from(board: Board) -> Self {
        Self {
            id: board.id,
            name: board.name,
            model: board.model,
            connected: board.connected,
            inner: board.inner,
        }
    }
}
