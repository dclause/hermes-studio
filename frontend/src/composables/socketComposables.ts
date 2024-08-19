import { io, type Socket } from 'socket.io-client';
import { useToasterStore } from '@/stores/toastStore';
import { SocketAck } from '@/types/socket';

declare type AckCallback = (ack: SocketAck) => void;

// The socket connexion.
let socketIO: Socket;

// Stores socket event registration method (@see connectionStore for usage example).
let socketEvents = new Set<(socket: Socket) => void>();

export function useSocketIO() {
  return {
    socketOpen: (url: string) => {
      socketIO = io(url) as Socket;

      socketIO.off();
      socketEvents.forEach((registerHandler) => {
        registerHandler(socketIO);
      });

      setTimeout(() => {
        if (socketIO.disconnected) {
          useToasterStore().error('Server connection failed.');
        }
      }, 5000);

      useToasterStore().success('Server connected.');
    },
    socketClose: () => {
      socketIO.close();
    },
    socketEmit: (event: string, ...args: unknown[]) => {
      const toaster = useToasterStore();
      // Extract optional callback.
      const callback: AckCallback | undefined =
        typeof args[args.length - 1] === 'function'
          ? (args[args.length - 1] as AckCallback)
          : undefined;
      if (callback) {
        args.pop();
      }
      return new Promise((resolve: AckCallback) => {
        socketIO.emit(event, ...args, (ack: SocketAck) => {
          if (ack.error) {
            toaster.error(ack.error);
          }
          if (callback) {
            callback(ack);
          }
          resolve(ack);
        });
      });
    },
    socketRegister: (registerHandler: (socket: Socket) => void) => {
      if (!socketEvents) {
        socketEvents = new Set<(socket: Socket) => void>();
      }
      socketEvents.add(registerHandler);
    },
  };
}
