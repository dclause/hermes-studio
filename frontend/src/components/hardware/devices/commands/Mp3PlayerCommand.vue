<template>
  <default-command :device="device" class="command-led">
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-led class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <v-select :items="fileInfos" item-title="name" hide-details density="compact" />
      <v-btn icon="mdi-play" variant="text" color="success" size="x-large" density="compact" />
      <v-btn icon="mdi-stop" variant="text" color="success" size="x-large" density="compact" />
      <v-btn icon="mdi-replay" variant="text" color="success" size="x-large" density="compact" />
    </template>
  </default-command>
</template>

<script lang="ts" setup async>
import { onBeforeMount, ref } from 'vue';
import { useFetchMp3PlayerFileList } from '@/composables/deviceComposables';
import { HardwareMode } from '@/composables/globalComposables';
import { DeviceState, Mp3Player, Mp3PlayerFile } from '@/types/devices';

const state = defineModel<DeviceState>({ required: true });
const props = withDefaults(
  defineProps<{
    device: Mp3Player;
    mode?: HardwareMode;
  }>(),
  { mode: HardwareMode.REALTIME },
);

const fileInfos = ref<Mp3PlayerFile[]>();
onBeforeMount(async () => {
  fileInfos.value = await useFetchMp3PlayerFileList(props.device);
});
</script>
