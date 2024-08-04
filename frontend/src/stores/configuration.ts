import { defineStore } from 'pinia';
import { type Socket } from 'socket.io-client';
import { WritableComputedRef } from 'vue';
import { useEmitter } from '@/composables/emitter';
import { emit } from '@/composables/socket';
import { useToasterStore } from '@/stores/toasts';
import { SocketAck } from '@/types/socket';

const STORAGE_KEY = 'hermes-locale';
const storedLanguage = sessionStorage.getItem(STORAGE_KEY);
const toaster = useToasterStore();

// Register socket events.
const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
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
  }),
  actions: {
    refresh() {
      emit('config:get', (ack: SocketAck) => {
        if (ack.success) {
          this.$patch(ack.success);
        }
      });
    },
    updateLanguage(i18n: WritableComputedRef<string>, locale: string) {
      this.locale = locale;
      i18n.value = locale;
      sessionStorage.setItem(STORAGE_KEY, locale);
      emit('config:set', this.$state, (ack: SocketAck) => {
        if (ack.success) {
          this.$patch(ack.success);
          toaster.success('Configuration saved');
        }
      });
    },
  },
});
