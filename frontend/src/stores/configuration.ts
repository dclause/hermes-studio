import { defineStore } from 'pinia';
import { type Socket } from 'socket.io-client';
import { WritableComputedRef } from 'vue';
import { useEmitter } from '@/composables/emitter';
import { useConnectionStore } from '@/stores/connection';
import { SocketAck } from '@/types/socket';

const STORAGE_KEY = 'hermes-locale';
const storedLanguage = sessionStorage.getItem(STORAGE_KEY);
const socket = useConnectionStore();

// Register socket events.
const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
  const store = useConfigStore();
  socket.on('connect', () => {
    socket.emit('config:get', (ack: SocketAck) => {
      if (ack.success) {
        store.$patch(ack.success);
      }
    });
  });
});

export const useConfigStore = defineStore({
  id: 'configuration',
  state: () => ({
    // Language configuration
    locale: storedLanguage ?? 'en',
  }),
  actions: {
    updateLanguage(i18n: WritableComputedRef<string>, locale: string) {
      this.locale = locale;
      i18n.value = locale;
      sessionStorage.setItem(STORAGE_KEY, locale);
      socket.emit('config:set', this.$state);
    },
  },
});
