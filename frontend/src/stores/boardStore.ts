// Register socket events.
import type { Board, BoardId, Protocol } from '@/types/boards';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { useSocketIO } from '@/composables/socketComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { useToasterStore } from '@/stores/toastStore';
import { SocketAck } from '@/types/socket';

const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
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
  socket.on('board:deleted', (board: Board) => {
    delete boardStore.boards[board.id];
  });
});

export const useBoardStore = defineStore({
  id: 'boards',
  state: () => ({
    loading: false,
    boards: {} as Record<BoardId, Board>,
  }),
  actions: {
    refresh() {
      this.loading = true;
      socketEmit('board:list', (ack: SocketAck) => {
        if (ack.success) {
          this.boards = ack.success as Record<BoardId, Board>;
        }
        this.loading = false;
      });
    },

    /**
     * Creates a new default board (without saving).
     */
    default(): Board {
      return {
        id: 0 as BoardId,
        name: 'New board',
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
    create(board: Board) {
      this.loading = true;
      return socketEmit('board:create', board, (ack: SocketAck) => {
        if (ack.success) {
          const createdBoard = ack.success as Board;
          this.boards[createdBoard.id] = createdBoard;
          useToasterStore().success(
            `Successfully created board '${createdBoard.name}' [${createdBoard.id}]`,
          );
        }
        this.loading = false;
      });
    },

    get(id: BoardId): Board {
      return this.boards[id];
    },

    delete(id: BoardId) {
      this.loading = true;
      return socketEmit('board:delete', id, (ack: SocketAck) => {
        if (ack.success) {
          const deletedBoard = ack.success as Board;
          delete this.boards[deletedBoard.id];
          useToasterStore().info(
            `Board '${deletedBoard.name}' [${deletedBoard.id}] as been deleted`,
          );
        }
        this.loading = false;
      });
    },

    open(id: BoardId) {
      return socketEmit('board:open', id, (ack: SocketAck) => {
        if (ack.success) {
          useDeviceStore().refresh();
          const board = ack.success as Board;
          this.boards[board.id] = board;
        }
      });
    },
    close(id: BoardId) {
      return socketEmit('board:close', id, (ack: SocketAck) => {
        if (ack.success) {
          const board = ack.success as Board;
          this.boards[board.id] = board;
        }
      });
    },

    reset(id: BoardId) {
      return socketEmit('board:reset', id);
    },

    reset_all() {
      Object.values(this.boards).forEach((board) => socketEmit('board:reset', board.id));
    },
  },
});
