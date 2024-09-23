// Register socket events.
import type { Board, BoardId, Protocol } from '@/types/boards';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { ArduinoType } from '@/components/hardware/boards/edit/ArduinoBoardEdit.vue';
import { BoardType } from '@/composables/boardComposables';
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
        model: { [BoardType.Arduino]: ArduinoType.OTHER } as unknown as BoardType,
        protocol: {
          type: 'SerialProtocol',
          port: 'COM3',
        } as Protocol,
        connected: false,
      };
    },

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

    update(board: Board) {
      this.loading = true;
      return socketEmit('board:update', board, (ack: SocketAck) => {
        if (ack.success) {
          const updatedBoard = ack.success as Board;
          this.boards[updatedBoard.id] = updatedBoard;
          useToasterStore().success(
            `Successfully update board '${updatedBoard.name}' [${updatedBoard.id}]`,
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
      this.boards[id].loading = true;
      return socketEmit('board:open', id, (ack: SocketAck) => {
        if (ack.success) {
          useDeviceStore().refresh();
          const board = ack.success as Board;
          this.boards[board.id] = board;
        }
        this.boards[id].loading = false;
      });
    },
    close(id: BoardId) {
      this.boards[id].loading = true;
      return socketEmit('board:close', id, (ack: SocketAck) => {
        if (ack.success) {
          const board = ack.success as Board;
          this.boards[board.id] = board;
        }
        this.boards[id].loading = false;
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
