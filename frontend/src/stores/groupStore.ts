import { defineStore } from 'pinia';
import { type Socket } from 'socket.io-client';
import { useSocketIO } from '@/composables/socketComposables';
import { useToasterStore } from '@/stores/toastStore';
import { FlatGroup, GroupId } from '@/types/groups';
import { SocketAck } from '@/types/socket';

const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
  const groupStore = useGroupStore();

  socket.on('connect', () => {
    groupStore.refresh();
  });

  socket.on('groups:updated', (groups: Record<GroupId, FlatGroup>) => {
    groupStore.groups = groups;
  });

  socket.on('group:deleted', (group: FlatGroup) => {
    delete groupStore.groups[group.id];
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
      socketEmit('group:list', (ack: SocketAck) => {
        if (ack.success) {
          this.groups = ack.success as Record<GroupId, FlatGroup>;
        }
      });
    },

    default(): FlatGroup {
      return {
        id: 0 as GroupId,
        name: 'New group',
        children: [],
        order: Object.values(this.groups).length,
      };
    },

    create(group: FlatGroup) {
      this.loading = true;
      return socketEmit('group:create', group, (ack: SocketAck) => {
        if (ack.success) {
          const createdGroup = ack.success as FlatGroup;
          this.groups[createdGroup.id] = createdGroup;
          useToasterStore().success(
            `Successfully created group '${createdGroup.name}' [${createdGroup.id}]`,
          );
        }
        this.loading = false;
      });
    },

    save(groups: Record<GroupId, FlatGroup>) {
      this.loading = true;
      return socketEmit('groups:update', groups, (ack: SocketAck) => {
        if (ack.success) {
          this.groups = ack.success as Record<GroupId, FlatGroup>;
        }
        this.loading = false;
      });
    },
    delete(id: GroupId) {
      this.loading = true;
      return socketEmit('group:delete', id, (ack: SocketAck) => {
        if (ack.success) {
          const deletedGroup = ack.success as FlatGroup;
          delete this.groups[deletedGroup.id];

          useToasterStore().info(
            `Group '${deletedGroup.name}' [${deletedGroup.id}] as been deleted`,
          );
        }
        this.loading = false;
      });
    },
  },
});
