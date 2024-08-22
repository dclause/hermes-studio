import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { WritableComputedRef } from 'vue';
import { HardwareMode } from '@/composables/globalComposables';
import { useSocketIO } from '@/composables/socketComposables';
import { useToasterStore } from '@/stores/toastStore';
import { SocketAck } from '@/types/socket';

const STORAGE_KEY = 'hermes-locale';
const storedLanguage = sessionStorage.getItem(STORAGE_KEY);
const toaster = useToasterStore();
const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
  const store = useConfigStore();
  socket.on('connect', () => {
    store.refresh();
  });
});

export const useConfigStore = defineStore({
  id: 'configuration',
  state: () => ({
    // Language configuration
    locale: storedLanguage ?? 'en',
    mode: HardwareMode.NONE,
  }),
  actions: {
    refresh() {
      socketEmit('config:get', (ack: SocketAck) => {
        if (ack.success) {
          this.$patch(ack.success);
        }
      });
    },
    updateLanguage(i18n: WritableComputedRef<string>, locale: string) {
      this.locale = locale;
      i18n.value = locale;
      sessionStorage.setItem(STORAGE_KEY, locale);
      socketEmit('config:set', this.$state, (ack: SocketAck) => {
        if (ack.success) {
          this.$patch(ack.success);
          toaster.success('Configuration saved');
        }
      });
    },
  },
});
