// Register socket events.
import type { BoardId, Expander, ExpanderId, HardwareId } from '@/types/hardwares';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { useSocketIO } from '@/composables/socketComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { useToasterStore } from '@/stores/toastStore';
import { SocketAck } from '@/types/socket';

const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
  const expanderStore = useExpanderStore();

  // React to socket being connected: get the expander list.
  socket.on('connect', () => {
    expanderStore.refresh();
  });

  // React to a new expander created: store it.
  socket.on('expander:created', (expander: Expander) => {
    expanderStore.expanders[expander.id] = expander;
  });

  // React to expander change: store it.
  socket.on('expander:updated', (expander: Expander) => {
    expanderStore.expanders[expander.id] = expander;
  });

  // React to expander deletion: remove it.
  socket.on('expander:deleted', (expander: Expander) => {
    delete expanderStore.expanders[expander.id];
  });
});

export const useExpanderStore = defineStore({
  id: 'expanders',
  state: () => ({
    loading: false,
    expanders: {} as Record<ExpanderId, Expander>,
  }),
  actions: {
    refresh() {
      this.loading = true;
      socketEmit('expander:list', (ack: SocketAck) => {
        if (ack.success) {
          this.expanders = ack.success as Record<ExpanderId, Expander>;
        }
        this.loading = false;
      });
    },

    /**
     * Creates a new default expander (without saving).
     */
    default(hid = 0): Expander {
      return {
        id: 0 as ExpanderId,
        name: 'New expander',
        type: 'Unknown',
        hid: { type: 'Board', id: hid } as HardwareId,
      };
    },

    create(expander: Expander) {
      this.loading = true;
      return socketEmit('expander:create', expander, (ack: SocketAck) => {
        if (ack.success) {
          const createdExpander = ack.success as Expander;
          this.expanders[createdExpander.id] = createdExpander;
          useToasterStore().success(
            `Successfully created expander '${createdExpander.name}' [${createdExpander.id}]`,
          );
        }
        this.loading = false;
      });
    },

    update(expander: Expander) {
      this.loading = true;
      return socketEmit('expander:update', expander, (ack: SocketAck) => {
        if (ack.success) {
          const updatedExpander = ack.success as Expander;
          this.expanders[updatedExpander.id] = updatedExpander;
          useToasterStore().success(
            `Successfully update expander '${updatedExpander.name}' [${updatedExpander.id}]`,
          );
        }
        this.loading = false;
      });
    },

    get(id: ExpanderId): Expander {
      return this.expanders[id];
    },

    /** List all expanders for given board */
    list_by_board(bid: BoardId): Expander[] {
      return Object.values(this.expanders).filter(
        (expander) => (expander.hid.id as BoardId) === bid,
      );
    },

    delete(id: ExpanderId) {
      this.loading = true;
      return socketEmit('expander:delete', id, (ack: SocketAck) => {
        if (ack.success) {
          const deletedExpander = ack.success as Expander;
          delete this.expanders[deletedExpander.id];
          useToasterStore().info(
            `Expander '${deletedExpander.name}' [${deletedExpander.id}] as been deleted`,
          );
        }
        this.loading = false;
      });
    },

    open(id: ExpanderId) {
      this.expanders[id].loading = true;
      return socketEmit('expander:open', id, (ack: SocketAck) => {
        if (ack.success) {
          useDeviceStore().refresh();
          const expander = ack.success as Expander;
          this.expanders[expander.id] = expander;
        }
        this.expanders[id].loading = false;
      });
    },
    close(id: ExpanderId) {
      this.expanders[id].loading = true;
      return socketEmit('expander:close', id, (ack: SocketAck) => {
        if (ack.success) {
          const expander = ack.success as Expander;
          this.expanders[expander.id] = expander;
        }
        this.expanders[id].loading = false;
      });
    },

    reset(id: ExpanderId) {
      return socketEmit('expander:reset', id);
    },

    reset_all() {
      return socketEmit('expander:reset_all');
    },
  },
});
