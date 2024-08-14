import { defineStore } from 'pinia';
import { type Socket } from 'socket.io-client';
import { useEmitter } from '@/composables/emitterComposables';
import { emit } from '@/composables/socketComposables';
import { FlatGroup, GroupId } from '@/types/groups';
import { SocketAck } from '@/types/socket';

// Register socket events.
const emitter = useEmitter();
emitter.on('socket:connected', (socket: Socket) => {
  const groupStore = useGroupStore();

  socket.on('connect', () => {
    groupStore.refresh();
  });

  socket.on('groups:updated', (groups: Record<GroupId, FlatGroup>) => {
    groupStore.groups = groups;
  });
});

export const useGroupStore = defineStore({
  id: 'groups',
  state: () => ({
    loading: false,
    groups: {} as Record<GroupId, FlatGroup>,
  }),
  actions: {
    refresh() {
      emit('group:list', (ack: SocketAck) => {
        if (ack.success) {
          this.groups = ack.success as Record<GroupId, FlatGroup>;
        }
      });
    },
    save(groups: Record<GroupId, FlatGroup>) {
      this.loading = true;
      return emit('groups:update', groups, (ack: SocketAck) => {
        if (ack.success) {
          this.groups = ack.success as Record<GroupId, FlatGroup>;
        }
        this.loading = false;
      });
    },
  },
});
