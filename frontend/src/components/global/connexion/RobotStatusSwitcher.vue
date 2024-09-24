<template>
  <v-switch
    inset
    :loading="loading"
    :disabled="loading || !isConnected"
    :model-value="status"
    :false-value="RobotStatus.OFF"
    :true-value="RobotStatus.ON"
    class="mode-switcher"
    :class="{ intermediate: status === 1 }"
    density="comfortable"
    hide-details
    @update:model-value="toggleBoards"
  >
    <template #prepend>
      <v-icon style="margin-right: -10px" icon="mdi-controller-off" @click="closeAllBoards" />
    </template>
    <template #append>
      <v-icon style="margin-left: -10px" icon="mdi-controller" @click="openAllBoards" />
    </template>
  </v-switch>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { RobotStatus, useBoardMode } from '@/composables/boardComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useConnectionStore } from '@/stores/connectionStore';

const boardStore = useBoardStore();
const loading = ref<boolean>(false);
const { boards } = storeToRefs(boardStore);

const status = useBoardMode();
const { isConnected } = storeToRefs(useConnectionStore());

const toggleBoards = () => {
  status.value === RobotStatus.OFF ? openAllBoards() : closeAllBoards();
};
const openAllBoards = () => {
  loading.value = true;
  Object.values(boards.value).map((board) =>
    boardStore.open(board.id).finally(() => (loading.value = false)),
  );
};
const closeAllBoards = () =>
  Object.values(boards.value).map((board) =>
    boardStore.close(board.id).finally(() => (loading.value = false)),
  );
</script>

<style lang="scss">
.mode-switcher {
  margin-right: 30px !important;

  &.intermediate {
    .v-selection-control__input {
      transform: translateX(0px) !important;
    }
  }
}
</style>
