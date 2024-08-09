// Register socket events.
import type { BoardId } from '@/types/boards';
import type { Actuator, DeviceId, DeviceState } from '@/types/devices';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { useEmitter } from '@/composables/emitterComposables';
import { emit } from '@/composables/socketComposables';
import { SocketAck } from '@/types/socket';

const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
  const deviceStore = useDeviceStore();

  // React to socket being connected: get the actuator list.
  socket.on('connect', () => {
    deviceStore.refresh();
  });

  // React to a new actuator created: store it.
  socket.on('actuator:created', (actuator: Actuator) => {
    deviceStore.actuators[actuator.id] = actuator;
  });

  // React to actuator change: store it.
  socket.on('actuator:updated', (actuator: Actuator) => {
    deviceStore.actuators[actuator.id] = actuator;
  });

  // React to actuator deletion: remove it.
  socket.on('actuator:deleted', (id: DeviceId) => {
    delete deviceStore.actuators[id];
  });
});

export const useDeviceStore = defineStore({
  id: 'actuators',
  state: () => ({
    loading: false,
    actuators: {} as Record<DeviceId, Actuator>,
  }),
  actions: {
    refresh() {
      this.loading = true;
      emit('actuator:list', (ack: SocketAck) => {
        if (ack.success) {
          this.actuators = ack.success as Record<DeviceId, Actuator>;
        }
        this.loading = false;
      });
    },

    /** List all devices for given board */
    list_by_board(bid: BoardId): Actuator[] {
      return Object.values(this.actuators).filter((actuator) => (actuator.board = bid));
    },

    /** Creates a new default actuator (without saving). */
    default(bid = null): Actuator {
      return {
        id: null,
        name: 'New actuator',
        type: null,
        bid,
        state: 0,
        default: 0,
      };
    },

    /**
     * Add a new actuator to the database.
     * @param actuator
     */
    add(actuator: Actuator) {
      this.loading = true;
      return emit('actuator:add', actuator, (ack: SocketAck) => {
        if (ack.success) {
          const createdDevice = ack.success as Actuator;
          this.actuators[createdDevice.id] = createdDevice;
        }
        this.loading = false;
      });
    },

    get(bid: DeviceId): Actuator {
      return this.actuators[bid];
    },

    mutate(id: DeviceId, state: DeviceState) {
      return emit('actuator:mutate', id, state);
    },
  },
});
