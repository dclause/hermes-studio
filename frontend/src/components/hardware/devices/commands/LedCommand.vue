<template>
  <default-command v-bind="$attrs" class="command-led">
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-led class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <boolean-action
        v-if="!keyframe"
        v-model="device.state"
        class="ml-2"
        :mode="mode"
        :device="device"
        :true="device.intensity"
        :false="0"
      />
      <boolean-action
        v-else
        v-model="keyframe.target"
        class="ml-2"
        :mode="mode"
        :device="device"
        :true="device.intensity"
        :false="0"
      />
    </template>
  </default-command>
</template>

<script lang="ts" setup>
import { HardwareMode } from '@/composables/globalComposables';
import { Keyframe } from '@/types/animation';
import { Led } from '@/types/devices';

const device = defineModel<Led>({ required: true });
const keyframe = defineModel<Keyframe>('keyframe', { required: false });
withDefaults(
  defineProps<{
    mode?: HardwareMode;
  }>(),
  { mode: HardwareMode.REALTIME },
);
</script>
