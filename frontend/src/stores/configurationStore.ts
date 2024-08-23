import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { HardwareMode } from '@/composables/globalComposables';
import { useSocketIO } from '@/composables/socketComposables';
import { SocketAck } from '@/types/socket';

const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
  const store = useConfigStore();
  socket.on('connect', () => {
    store.refresh();
  });

  socket.on('config:updated', (ack: SocketAck) => {
    store.$patch(ack as any);
  });
});

export const useConfigStore = defineStore({
  id: 'configuration',
  state: () => ({
    locale: 'en',
    mode: HardwareMode.OFF,
  }),
  actions: {
    refresh() {
      socketEmit('config:get', (ack: SocketAck) => {
        if (ack.success) {
          this.$patch(ack.success);
        }
      });
    },
    update(partial: any) {
      socketEmit('config:set', { ...this.$state, ...partial }, (ack: SocketAck) => {
        if (ack.success) {
          this.$patch(ack.success);
        }
      });
    },
  },
});
