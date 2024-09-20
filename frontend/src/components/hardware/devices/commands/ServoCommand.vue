<template>
  <default-command :device="device" class="command-servo">
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-servo class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <servo-action
        v-model="state"
        :mode="mode"
        :device="device"
        :min="device.range[0]"
        :max="device.range[1]"
      />
    </template>
  </default-command>
</template>

<script lang="ts" setup>
import { HardwareMode } from '@/composables/globalComposables';
import { Servo } from '@/types/devices'; // const device = defineModel<Servo>({ required: true });

const state = defineModel<number>({ required: true });
withDefaults(
  defineProps<{
    device: Servo;
    mode?: HardwareMode;
  }>(),
  { mode: HardwareMode.REALTIME },
);
</script>
