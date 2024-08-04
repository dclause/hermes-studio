export type CallbackType = (...args: unknown[]) => void;

export declare interface SocketAck {
  success: unknown;
  error: string;
}
