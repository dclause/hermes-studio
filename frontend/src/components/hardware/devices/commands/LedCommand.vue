<template>
  <default-command :device="device" class="command-led" :mode="mode" :variant="variant">
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-led class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <boolean-action
        v-model="state as boolean"
        class="ml-2"
        :device="device"
        :mode="mode"
        :variant="variant"
      />
    </template>
  </default-command>
</template>

<script lang="ts" setup>
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { Led } from '@/types/devices';

const state = defineModel<boolean | number>({ required: true });
withDefaults(
  defineProps<{
    device: Led;
    mode?: HardwareMode;
    variant?: CommandMode;
  }>(),
  { mode: HardwareMode.REALTIME, variant: CommandMode.FULL },
);
</script>
