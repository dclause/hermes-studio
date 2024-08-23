<template>
  <v-switch
    v-model="state"
    color="primary"
    density="compact"
    :disabled="loading || mode === HardwareMode.OFF"
    hide-details
    inline
    inset
    :loading="loading"
  />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { HardwareMode, logError } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useConfigStore } from '@/stores/configurationStore';
import { Board } from '@/types/boards';

const boardStore = useBoardStore();
const board = defineModel<Board>({ required: true });
const loading = ref<boolean>(false);
const { mode } = storeToRefs(useConfigStore());

const state = computed({
  get() {
    return board.value.connected;
  },
  set(value) {
    loading.value = true;
    if (mode.value === HardwareMode.REALTIME) {
      boardStore[value ? 'open' : 'close'](board.value)
        .then((): void => {
          loading.value = false;
          return;
        })
        .catch(logError);
    }
  },
});
</script>
