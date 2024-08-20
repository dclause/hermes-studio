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

const boardStore = useBoardStore();
const board = defineModel<Board>({ required: true });
const loading = ref<boolean>(false);

const state = computed({
  get() {
    return board.value.connected;
  },
  set(value) {
    loading.value = true;
    boardStore[value ? 'open' : 'close'](board.value)
      .then((): void => {
        loading.value = false;
        return;
      })
      .catch(logError);
  },
});
</script>
