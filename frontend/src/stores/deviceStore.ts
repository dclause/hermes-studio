// Register socket events.
import type { BoardId } from '@/types/boards';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { DeviceType } from '@/composables/deviceComposables';
import { useEmitter } from '@/composables/emitterComposables';
import { emit } from '@/composables/socketComposables';
import { useToasterStore } from '@/stores/toastStore';
import { Device, DeviceId, DeviceState } from '@/types/devices';
import { SocketAck } from '@/types/socket';

const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
  const deviceStore = useDeviceStore();

  // React to socket being connected: get the actuator list.
  socket.on('connect', () => {
    deviceStore.refresh();
  });

  // React to a new actuator created: store it.
  socket.on('device:created', (actuator: Device) => {
    deviceStore.devices[actuator.id] = actuator;
  });

  // React to actuator change: store it.
  socket.on('device:updated', (actuator: Device) => {
    deviceStore.devices[actuator.id] = actuator;
  });

  // React to actuator deletion: remove it.
  socket.on('device:deleted', (device: Device) => {
    delete deviceStore.devices[device.id];
  });

  // React to actuator mutate change: store it.
  socket.on('device:mutated', (id: DeviceId, state: DeviceState) => {
    deviceStore.devices[id].state = state;
  });
});

export const useDeviceStore = defineStore({
  id: 'devices',
  state: () => ({
    loading: false,
    devices: {} as Record<DeviceId, Device>,
  }),
  actions: {
    refresh() {
      this.loading = true;
      emit('device:list', (ack: SocketAck) => {
        if (ack.success) {
          this.devices = ack.success as Record<DeviceId, Device>;
        }
        this.loading = false;
      });
    },

    /** List all devices for given board */
    list_by_board(bid: BoardId): Device[] {
      return Object.values(this.devices).filter((device) => (device.board = bid));
    },

    /** Creates a new default actuator (without saving). */
    default(bid = null): Device {
      return {
        entity: 'Device',
        id: 0 as DeviceId,
        name: 'New device',
        type: DeviceType.Unknown,
        bid,
        pin: 13,
        state: 0,
        default: 0,
      };
    },

    /**
     * Add a new actuator to the database.
     * @param device
     */
    create(device: Device) {
      this.loading = true;
      return emit('device:create', device, (ack: SocketAck) => {
        if (ack.success) {
          const createdDevice = ack.success as Device;
          this.devices[createdDevice.id] = createdDevice;
        }
        this.loading = false;
      });
    },

    get(bid: DeviceId): Device {
      return this.devices[bid];
    },

    delete(id: DeviceId) {
      this.loading = true;
      return emit('device:delete', id, (ack: SocketAck) => {
        if (ack.success) {
          const deletedDevice = ack.success as Device;
          delete this.devices[deletedDevice.id];

          useToasterStore().info(
            `Device '${deletedDevice.name}' [${deletedDevice.id}] as been deleted`,
          );
        }
        this.loading = false;
      });
    },

    mutate(id: DeviceId, state: DeviceState) {
      return emit('device:mutate', id, state);
    },
  },
});
