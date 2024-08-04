import { Socket } from 'socket.io-client';

export type Events = {
  'socket:connected': Socket;
  'socket:disconnected': void;
};
