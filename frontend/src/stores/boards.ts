// Register socket events.
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { useEmitter } from '@/composables/emitter';
import { emit } from '@/composables/socket';
import { Board, BoardId, Protocol } from '@/types/hardware';
import { SocketAck } from '@/types/socket';

const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
  const boardStore = useBoardStore();

  // React to socket being connected: get the board list.
  socket.on('connect', () => {
    boardStore.refresh();
  });

  // React to a new board created: store it.
  socket.on('board:created', (board: Board) => {
    boardStore.boards[board.id] = board;
  });

  // React to board change: store it.
  socket.on('board:updated', (board: Board) => {
    boardStore.boards[board.id] = board;
  });

  // React to board deletion: remove it.
  socket.on('board:deleted', (id: BoardId) => {
    delete boardStore.boards[id];
  });
});

export const useBoardStore = defineStore({
  id: 'boards',
  state: () => ({
    loading: false,
    boards: [] as Record<BoardId, Board>,
  }),
  actions: {
    refresh() {
      this.loading = true;
      emit('board:list', (ack: SocketAck) => {
        if (ack.success) {
          this.boards = ack.success as Record<BoardId, Board>;
        }
        this.loading = false;
      });
    },

    /**
     * Creates a new default board (without saving).
     */
    create(): Board {
      return {
        id: 0 as BoardId,
        name: 'foobar',
        model: 'Unknown',
        protocol: {
          type: 'SerialProtocol',
          port: 'COM3',
        } as Protocol,
        connected: false,
      };
    },

    /**
     * Add a new board to the database.
     * @param board
     */
    add(board: Board) {
      this.loading = true;
      return emit('board:add', board, (ack: SocketAck) => {
        if (ack.success) {
          const createdBoard = ack.success as Board;
          this.boards[createdBoard.id] = createdBoard;
        }
        this.loading = false;
      });
    },

    get(bid: BoardId): Board {
      return this.boards[bid];
    },

    open(board: Board) {
      return emit('board:open', board.id);
    },
    close(board: Board) {
      return emit('board:close', board.id);
    },
  },
});
