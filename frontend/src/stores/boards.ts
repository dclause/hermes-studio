// Register socket events.
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { useEmitter } from '@/composables/emitter';
import { emit } from '@/composables/socket';
import { Board, BoardId } from '@/types/hardware';
import { SocketAck } from '@/types/socket';

const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
  const boardStore = useBoardStore();
  socket.on('connect', () => {
    console.log('refresh boards');
    boardStore.refresh();
  });
});

export const useBoardStore = defineStore({
  id: 'hardware',
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
        name: '',
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
  },
});
