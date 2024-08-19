<template>
  <v-switch
    v-model="state"
    color="primary"
    density="compact"
    :disabled="loading"
    hide-details
    inline
    inset
    :loading="loading"
  />
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { logError } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { Board } from '@/types/boards';
import { SocketAck } from '@/types/socket';

const boardStore = useBoardStore();
const board = defineModel<Board>({ required: true });
const loading = ref<boolean>(false);

/**
 * Mutates the connected computed with validation from the server.
 * If the mutation goes wrong: returns the state value to previous.
 */
let oldValue = board.value.connected;
const state = computed({
  get() {
    return board.value.connected;
  },
  set(value) {
    loading.value = true;
    oldValue = state.value;
    boardStore[value ? 'open' : 'close'](board.value)
      .then((ack: SocketAck): void => {
        if (ack.error) {
          board.value.connected = oldValue;
        }
        loading.value = false;
        return;
      })
      .catch(logError);
    board.value.connected = value;
  },
});
</script>
