// Register socket events.
import type { BoardId, ExpanderId, HardwareId } from '@/types/hardwares';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { DeviceType } from '@/composables/deviceComposables';
import { HardwareType } from '@/composables/hardwareComposables';
import { useSocketIO } from '@/composables/socketComposables';
import { Easing } from '@/composables/timelineComposables';
import { useToasterStore } from '@/stores/toastStore';
import { Device, DeviceId, DeviceState } from '@/types/devices';
import { SocketAck } from '@/types/socket';

const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
  const deviceStore = useDeviceStore();

  // React to socket being connected: get the actuator list.
  socket.on('connect', () => {
    deviceStore.refresh();
  });

  socket.on('device:list', (devices: Record<DeviceId, Device>) => {
    deviceStore.devices = devices;
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
      socketEmit('device:list', (ack: SocketAck) => {
        if (ack.success) {
          this.devices = ack.success as Record<DeviceId, Device>;
        }
        this.loading = false;
      });
    },

    /** List all devices for given hardware */
    list_by_hardware(hid: HardwareId): Device[] {
      return Object.values(this.devices).filter(
        (device) => device.hid.type === hid.type && device.hid.id === hid.id,
      );
    },

    /** List all devices for given board */
    list_by_board(hid: BoardId): Device[] {
      return Object.values(this.devices).filter(
        (device) => device.hid.type === HardwareType.Board && (device.hid.id as BoardId) === hid,
      );
    },
    /** List all devices for given expander */
    list_by_expander(hid: ExpanderId): Device[] {
      return Object.values(this.devices).filter(
        (device) =>
          device.hid.type === HardwareType.Expander && (device.hid.id as ExpanderId) === hid,
      );
    },

    /** Creates a new default actuator (without saving). */
    default(hid = 0): Device {
      return {
        entity: 'Device',
        id: 0 as DeviceId,
        name: 'New device',
        type: DeviceType.Led,
        hid: { type: 'Board', id: hid } as HardwareId,
        pin: 13,
        state: 0,
        default: 0,
      };
    },

    create(device: Device) {
      this.loading = true;
      return socketEmit('device:create', device, (ack: SocketAck) => {
        if (ack.success) {
          const createdDevice = ack.success as Device;
          this.devices[createdDevice.id] = createdDevice;
          useToasterStore().success(
            `Successfully created device '${createdDevice.name}' [${createdDevice.id}]`,
          );
        }
        this.loading = false;
      });
    },

    update(device: Device) {
      this.loading = true;
      return socketEmit('device:update', device, (ack: SocketAck) => {
        if (ack.success) {
          const updatedDevice = ack.success as Device;
          this.devices[updatedDevice.id] = updatedDevice;
          useToasterStore().success(
            `Successfully update device '${updatedDevice.name}' [${updatedDevice.id}]`,
          );
        }
        this.loading = false;
      });
    },

    get(hid: DeviceId): Device {
      return this.devices[hid];
    },

    delete(id: DeviceId) {
      this.loading = true;
      return socketEmit('device:delete', id, (ack: SocketAck) => {
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

    reset(id: DeviceId) {
      return socketEmit('device:reset', id);
    },

    mutate(id: DeviceId, state: DeviceState) {
      return socketEmit('device:mutate', id, state);
    },

    animate(id: DeviceId, state: DeviceState, duration: number, transition: Easing) {
      return socketEmit('device:animate', id, state, duration, transition, (ack: SocketAck) => {
        if (ack.success) {
          this.devices[id].state = ack.success;
        }
      });
    },
  },
});
