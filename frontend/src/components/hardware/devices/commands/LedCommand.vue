<template>
  <default-command :device="device" class="command-led">
    <template #prefix>
      <slot name="prefix" />
    </template>
    <template #icon>
      <svg-led class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <boolean-action v-model="state as boolean" class="ml-2" :mode="mode" :device="device" />
    </template>
  </default-command>
</template>

<script lang="ts" setup>
import { HardwareMode } from '@/composables/globalComposables';
import { Led } from '@/types/devices';

const state = defineModel<boolean | number>({ required: true });
withDefaults(
  defineProps<{
    device: Led;
    mode?: HardwareMode;
  }>(),
  { mode: HardwareMode.REALTIME },
);
</script>
