<template>
  <div
    class="text-subtitle-2"
    :class="{ disabled: !board?.connected }"
    :style="`padding-left:${track.level! * 40 + 16}px`"
  >
    <v-icon v-if="!board?.connected" icon="mdi-link-off" size="small" class="mr-1" />
    <span>{{ track.name ?? device?.name }}</span>

    <v-tooltip v-if="!board?.connected" activator="parent" location="top">
      Board is not connected
    </v-tooltip>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';
import { Track } from '@/types/timeline';

const boardStore = useBoardStore();
const deviceStore = useDeviceStore();
const track = defineModel<Track>({ required: true });
const device = computed(() => deviceStore.get(track.value.device!));
const board = computed(() => boardStore.get(device.value.bid));
</script>
<style lang="scss" scoped>
.disabled span {
  text-decoration: line-through;
}
</style>
