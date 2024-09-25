<template>
  <default-command
    :device="device"
    class="command-mp3"
    :mode="mode"
    :variant="variant"
    @reset="onReset"
  >
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <v-icon icon="mdi-music" class="ml-2 mr-3" width="30" />
    </template>
    <template #info>
      {{}}
    </template>
    <template #command>
      <mp3-player-action
        v-model="state"
        :device="device"
        :files="fileInfos"
        :mode="mode"
        :variant="variant"
      />
    </template>
  </default-command>
</template>

<script lang="ts" setup async>
import { onBeforeMount, ref } from 'vue';
import Mp3PlayerAction from '@/components/commands/Mp3PlayerAction.vue';
import { useFetchMp3PlayerFileList } from '@/composables/deviceComposables';
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { DeviceState, Mp3Player, Mp3PlayerFile, Mp3PlayerState } from '@/types/devices';

const state = defineModel<Mp3PlayerState>({ required: true });
const props = withDefaults(
  defineProps<{
    device: Mp3Player;
    mode?: HardwareMode;
    variant?: CommandMode;
  }>(),
  { mode: HardwareMode.REALTIME, variant: CommandMode.FULL },
);

// Retrieve the mp3Player file info.
const fileInfos = ref<Mp3PlayerFile[]>([]);
onBeforeMount(async () => {
  fileInfos.value = await useFetchMp3PlayerFileList(props.device);
});

const onReset = (value: DeviceState) => {
  state.value = value as Mp3PlayerState;
};
</script>
