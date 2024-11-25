<template>
  <default-command
    :device="device"
    class="command-servo"
    :mode="mode"
    :variant="variant"
    @reset="onReset"
  >
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-servo class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <servo-action
        v-model="state"
        :device="device"
        :min="device.range[0]"
        :max="device.range[1]"
        :mode="mode"
        :variant="variant"
      />
    </template>
  </default-command>
</template>

<script lang="ts" setup>
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { DeviceState, Servo } from '@/types/devices';

const state = defineModel<number>({ required: true });
withDefaults(
  defineProps<{
    device: Servo;
    mode?: HardwareMode;
    variant?: CommandMode;
  }>(),
  { mode: HardwareMode.REALTIME, variant: CommandMode.FULL },
);

const onReset = (value: DeviceState) => {
  state.value = value as number;
};
</script>
