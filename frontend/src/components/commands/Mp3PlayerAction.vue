<template>
  <generic-action
    v-model="state"
    class="action-boolean"
    :device="device"
    :mode="mode"
    :variant="variant"
  >
    <template #action="{ isCommandable }">
      <v-select
        v-model="state.path"
        :items="files"
        item-title="name"
        item-value="path"
        hide-details
        density="compact"
        @update:model-value="onCommand"
      />
      <v-btn
        v-if="isCommandable"
        :icon="state.status <= 0 ? 'mdi-play' : 'mdi-pause'"
        variant="text"
        color="success"
        size="x-large"
        density="compact"
        :disabled="!state.path"
        @click="state.status <= 0 ? onCommand(1) : onCommand(0)"
      />
      <v-btn
        v-if="isCommandable"
        :disabled="state.status < 0"
        icon="mdi-stop"
        variant="text"
        color="success"
        size="x-large"
        density="compact"
        @click="onCommand(-1)"
      />
    </template>
  </generic-action>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { CommandMode, HardwareMode, logError } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { Actuator, DeviceState, Mp3PlayerFile, Mp3PlayerState } from '@/types/devices';
import { SocketAck } from '@/types/socket';

const state = defineModel<Mp3PlayerState>({ required: true });

const props = withDefaults(
  defineProps<{
    mode?: HardwareMode;
    variant?: CommandMode;
    device: Actuator;
    files: Mp3PlayerFile[];
  }>(),
  {
    mode: HardwareMode.REALTIME,
    variant: CommandMode.FULL,
  },
);

const deviceStore = useDeviceStore();
const loading = ref<boolean>(false);

const onCommand = (command: string | number) => {
  let newState: DeviceState = command;
  if (props.mode === HardwareMode.REALTIME) {
    loading.value = true;
    if (props.variant === CommandMode.KEYFRAME && typeof command === 'string') {
      newState = { path: command, status: 2 };
    }
    deviceStore
      .mutate(props.device.id, newState)
      .then((ack: SocketAck) => {
        if (ack.success) {
          state.value = ack.success as Mp3PlayerState;
        }
        return;
      })
      .finally(() => (loading.value = false))
      .catch(logError);
  }
};
</script>
