<template>
  <v-switch
    v-model="state"
    color="primary"
    density="compact"
    :disabled="board.loading || mode === HardwareMode.OFF"
    hide-details
    inline
    inset
    :loading="board.loading"
  />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { HardwareMode, logError } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useConfigStore } from '@/stores/configurationStore';
import { Board } from '@/types/boards';

const boardStore = useBoardStore();
const board = defineModel<Board>({ required: true });
const { mode } = storeToRefs(useConfigStore());

const state = computed({
  get() {
    return board.value.connected;
  },
  set(value) {
    if (mode.value === HardwareMode.REALTIME) {
      boardStore[value ? 'open' : 'close'](board.value.id).catch(logError);
    }
  },
});
</script>
