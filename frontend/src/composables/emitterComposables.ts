import type { Events } from '@/types/events';
import type { Emitter } from 'mitt';
import mitt from 'mitt';

const emitter: Emitter<Events> = mitt();

export const useEmitter = () => emitter;
