import { defineStore, storeToRefs } from 'pinia';
import { Socket } from 'socket.io-client';
import { useSocketIO } from '@/composables/socketComposables';

const STORAGE_KEY = 'hermes-backend';
const storedUrl = sessionStorage.getItem(STORAGE_KEY);
const defaultUrl = import.meta.env.PROD ? `${window.location.host}/ws` : `localhost:4000/ws`;

const { socketOpen, socketClose, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
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
  }),

  actions: {
    /**
     * Stores the connexion parameters in the storage.
     */
    open() {
      sessionStorage.setItem(STORAGE_KEY, this.url);
      socketOpen(this.url);
    },

    /**
     * Stores the connexion parameters in the storage.
     */
    close() {
      socketClose();
    },
  },
});
