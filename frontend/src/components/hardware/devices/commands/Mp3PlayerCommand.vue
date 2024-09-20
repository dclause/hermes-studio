<template>
  <default-command :device="device" class="command-led">
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-led class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <mp3-player-action v-model="state" :mode="mode" :device="device" :files="fileInfos" />
    </template>
  </default-command>
</template>

<script lang="ts" setup async>
import { onBeforeMount, ref } from 'vue';
import Mp3PlayerAction from '@/components/commands/Mp3PlayerAction.vue';
import { useFetchMp3PlayerFileList } from '@/composables/deviceComposables';
import { HardwareMode } from '@/composables/globalComposables';
import { Mp3Player, Mp3PlayerFile, Mp3PlayerState } from '@/types/devices';

const state = defineModel<Mp3PlayerState>({ required: true });
const props = withDefaults(
  defineProps<{
    device: Mp3Player;
    mode?: HardwareMode;
  }>(),
  { mode: HardwareMode.REALTIME },
);

// Retrieve the mp3Player file info.
const fileInfos = ref<Mp3PlayerFile[]>([]);
onBeforeMount(async () => {
  fileInfos.value = await useFetchMp3PlayerFileList(props.device);
});
</script>
