import { useConnectionStore } from '@/stores/connectionStore';
import { useToasterStore } from '@/stores/toastStore';
import { SocketAck } from '@/types/socket';

declare type AckCallback = (ack: SocketAck) => void;

export function emit(event: string, ...args: unknown[]) {
  const toaster = useToasterStore();
  const socket = useConnectionStore();
  // Extract optional callback.
  const callback: AckCallback | undefined =
    typeof args[args.length - 1] === 'function'
      ? (args[args.length - 1] as AckCallback)
      : undefined;
  if (callback) {
    args.pop();
  }
  return new Promise((resolve: AckCallback) => {
    socket.emit(event, ...args, (ack: SocketAck) => {
      if (ack.error) {
        toaster.error(ack.error);
      }
      if (callback) {
        callback(ack);
      }
      resolve(ack);
    });
  });
}
