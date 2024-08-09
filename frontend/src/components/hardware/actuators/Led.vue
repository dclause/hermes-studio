<template>
  <default-actuator v-model="device" class="command command-led">
    <template #icon>
      <svg-led class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <boolean-action
        v-model="device.state"
        :mode="HardwareMode.REALTIME"
        :device="device"
        :true="device.intensity"
        :false="0"
      />
    </template>
  </default-actuator>
</template>

<script lang="ts" setup>
import { HardwareMode } from '@/composables/globalComposables';
import { Led } from '@/types/devices';

const device = defineModel<Led>({ required: true });
</script>
