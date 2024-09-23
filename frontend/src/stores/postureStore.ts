// Register socket events.
import type { Posture, PostureId } from '@/types/postures';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { useSocketIO } from '@/composables/socketComposables';
import { useToasterStore } from '@/stores/toastStore';
import { Position } from '@/types/animations';
import { SocketAck } from '@/types/socket';

const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
  const postureStore = usePostureStore();

  // React to socket being connected: get the posture list.
  socket.on('connect', () => {
    postureStore.refresh();
  });

  // React to a new posture created: store it.
  socket.on('posture:created', (posture: Posture) => {
    postureStore.postures[posture.id] = posture;
  });

  // React to posture change: store it.
  socket.on('posture:updated', (posture: Posture) => {
    postureStore.postures[posture.id] = posture;
  });

  // React to posture deletion: remove it.
  socket.on('posture:deleted', (posture: Posture) => {
    delete postureStore.postures[posture.id];
  });
});

export const usePostureStore = defineStore({
  id: 'postures',
  state: () => ({
    loading: false,
    postures: {} as Record<PostureId, Posture>,
  }),
  actions: {
    refresh() {
      this.loading = true;
      socketEmit('posture:list', (ack: SocketAck) => {
        if (ack.success) {
          this.postures = ack.success as Record<PostureId, Posture>;
        }
        this.loading = false;
      });
    },

    /**
     * Creates a new default posture (without saving).
     */
    default(): Posture {
      return {
        id: 0 as PostureId,
        name: 'New posture',
        description: '',
        positions: [] as Position[],
      };
    },

    create(posture: Posture) {
      this.loading = true;
      return socketEmit('posture:create', posture, (ack: SocketAck) => {
        if (ack.success) {
          const createdPosture = ack.success as Posture;
          this.postures[createdPosture.id] = createdPosture;
          useToasterStore().success(
            `Successfully created posture '${createdPosture.name}' [${createdPosture.id}]`,
          );
        }
        this.loading = false;
      });
    },

    update(posture: Posture) {
      this.loading = true;
      return socketEmit('posture:update', posture, (ack: SocketAck) => {
        if (ack.success) {
          const updatedPosture = ack.success as Posture;
          this.postures[updatedPosture.id] = updatedPosture;
          useToasterStore().success(
            `Successfully updated posture '${updatedPosture.name}' [${updatedPosture.id}]`,
          );
        }
        this.loading = false;
      });
    },

    get(id: PostureId): Posture {
      return this.postures[id];
    },

    delete(id: PostureId) {
      this.loading = true;
      return socketEmit('posture:delete', id, (ack: SocketAck) => {
        if (ack.success) {
          const deletedPosture = ack.success as Posture;
          delete this.postures[deletedPosture.id];
          useToasterStore().info(
            `Posture '${deletedPosture.name}' [${deletedPosture.id}] as been deleted`,
          );
        }
        this.loading = false;
      });
    },

    play(posture: Posture) {
      return socketEmit('posture:play', posture.id);
    },
  },
});
