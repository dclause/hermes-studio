<template>
  <default-command v-model="device" class="command-servo">
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-servo class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <servo-action
        v-if="!keyframe"
        v-model="device.state"
        :mode="mode"
        :device="device"
        :min="device.range[0]"
        :max="device.range[1]"
      />
      <servo-action
        v-else
        v-model="keyframe.target"
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
import { Keyframe } from '@/types/animation';
import { Servo } from '@/types/devices';

const device = defineModel<Servo>({ required: true });
const keyframe = defineModel<Keyframe>('keyframe', { required: false });
withDefaults(
  defineProps<{
    mode?: HardwareMode;
  }>(),
  { mode: HardwareMode.REALTIME },
);
</script>
