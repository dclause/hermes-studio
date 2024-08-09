import type { CallbackType } from '@/types/socket';
import { defineStore, storeToRefs } from 'pinia';
import { io, type Socket } from 'socket.io-client';
import { useEmitter } from '@/composables/emitterComposables';

const STORAGE_KEY = 'hermes-backend';
const storedUrl = sessionStorage.getItem(STORAGE_KEY);
const defaultUrl = import.meta.env.PROD ? `${window.location.host}/ws` : `localhost:4000/ws`;

// Register socket events.
const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
  const store = useConnectionStore();
  const { isConnected } = storeToRefs(store);
  socket.on('connect', () => {
    isConnected.value = true;
  });
  socket.on('reconnect_attempt', () => {
    isConnected.value = undefined;
  });
  socket.on('disconnect', () => {
    store.isConnected = false;
    emitter.emit('socket:disconnected');
  });
});

export const useConnectionStore = defineStore({
  id: 'connection',

  state: () => ({
    // Url to the socket API.
    url: storedUrl ?? defaultUrl,
    // Indicates if the socket to the webserver is currently connected.
    // true/false, undefined for pending.
    isConnected: false as boolean | undefined,
    // Actual socketIO client
    socket: io(storedUrl ?? defaultUrl),
  }),

  actions: {
    /**
     * Stores the connexion parameters in the storage.
     */
    open() {
      sessionStorage.setItem(STORAGE_KEY, this.url);
      this.socket = io(this.url);

      setTimeout(() => {
        if (this.socket.disconnected) {
          emitter.emit('socket:disconnected');
        }
      }, 5000);

      // Clear all socket bindings (only necessary for hot reload)
      this.socket.off();

      // Add all socket events bindings.
      emitter.emit('socket:connected', this.socket as Socket);
    },

    /**
     * Stores the connexion parameters in the storage.
     */
    close() {
      this.socket.close();
    },

    /**
     * Pass through socket event binding.
     */
    on(event: string, callback: CallbackType) {
      this.socket.on(event, callback);
    },

    /**
     * Pass through socket emit.
     */
    emit(event: string, ...args: unknown[]) {
      this.socket.emit(event, ...args);
    },
  },
});
