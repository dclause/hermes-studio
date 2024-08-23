// Register socket events.
import type { Animation, AnimationId } from '@/types/animation';
import { defineStore } from 'pinia';
import { Socket } from 'socket.io-client';
import { useSocketIO } from '@/composables/socketComposables';
import { useToasterStore } from '@/stores/toastStore';
import { SocketAck } from '@/types/socket';

const { socketEmit, socketRegister } = useSocketIO();

// Register socket events.
socketRegister((socket: Socket) => {
  const animationStore = useAnimationStore();

  // React to socket being connected: get the animation list.
  socket.on('connect', () => {
    animationStore.refresh();
  });

  // React to a new animation created: store it.
  socket.on('animation:created', (animation: Animation) => {
    animationStore.animations[animation.id] = animation;
  });

  // React to animation change: store it.
  socket.on('animation:updated', (animation: Animation) => {
    animationStore.animations[animation.id] = animation;
  });

  // React to animation deletion: remove it.
  socket.on('animation:deleted', (animation: Animation) => {
    delete animationStore.animations[animation.id];
  });
});

export const useAnimationStore = defineStore({
  id: 'animations',
  state: () => ({
    loading: false,
    animations: {} as Record<AnimationId, Animation>,
    timer: 0,
  }),
  actions: {
    refresh() {
      this.loading = true;
      socketEmit('animation:list', (ack: SocketAck) => {
        if (ack.success) {
          this.animations = ack.success as Record<AnimationId, Animation>;
        }
        this.loading = false;
      });
    },

    /**
     * Creates a new default animation (without saving).
     */
    default(): Animation {
      return {
        id: 0 as AnimationId,
        name: 'New animation',
        description: '',
        repeat: false,
        loopback: 0,
        fps: 60,
        speed: 100,
        keyframes: {},
        duration: 0,
        playing: 0,
        progress: 0,
      };
    },

    create(animation: Animation) {
      this.loading = true;
      return socketEmit('animation:create', animation, (ack: SocketAck) => {
        if (ack.success) {
          const createdAnimation = ack.success as Animation;
          this.animations[createdAnimation.id] = createdAnimation;
          useToasterStore().success(
            `Successfully created animation '${createdAnimation.name}' [${createdAnimation.id}]`,
          );
        }
        this.loading = false;
      });
    },

    update(animation: Animation) {
      this.loading = true;
      return socketEmit('animation:update', animation, (ack: SocketAck) => {
        if (ack.success) {
          const updatedAnimation = ack.success as Animation;
          this.animations[updatedAnimation.id] = updatedAnimation;
          useToasterStore().success(
            `Successfully updated animation '${updatedAnimation.name}' [${updatedAnimation.id}]`,
          );
        }
        this.loading = false;
      });
    },

    get(id: AnimationId): Animation {
      return this.animations[id];
    },

    delete(id: AnimationId) {
      this.loading = true;
      return socketEmit('animation:delete', id, (ack: SocketAck) => {
        if (ack.success) {
          const deletedAnimation = ack.success as Animation;
          delete this.animations[deletedAnimation.id];
          useToasterStore().info(
            `Animation '${deletedAnimation.name}' [${deletedAnimation.id}] as been deleted`,
          );
        }
        this.loading = false;
      });
    },

    play(animation: Animation) {
      this.loading = true;
      return socketEmit('animation:play', animation.id, (ack: SocketAck) => {
        if (ack.success) {
          const updatedAnimation = ack.success as Animation;
          this.animations[updatedAnimation.id] = updatedAnimation;
        }
        this.loading = false;
      });
    },

    stop(animation: Animation) {
      this.loading = true;
      socketEmit('animation:stop', animation.id, (ack: SocketAck) => {
        if (ack.success) {
          const updatedAnimation = ack.success as Animation;
          this.animations[updatedAnimation.id] = updatedAnimation;
        }
        this.loading = false;
      });
    },

    pause(animation: Animation) {
      this.loading = true;
      socketEmit('animation:pause', animation.id, (ack: SocketAck) => {
        if (ack.success) {
          const updatedAnimation = ack.success as Animation;
          this.animations[updatedAnimation.id] = updatedAnimation;
        }
        this.loading = false;
      });
    },
  },
});
