<template>
  <default-command v-model="device" class="command-servo">
    <template #icon>
      <svg-servo class="ml-2 mr-3" width="30" />
    </template>
    <template #command>
      <servo-action
        v-model="device.state"
        :mode="HardwareMode.REALTIME"
        :device="device"
        :min="device.range[0]"
        :max="device.range[1]"
      />
    </template>
  </default-command>
</template>

<script lang="ts" setup>
import { HardwareMode } from '@/composables/globalComposables';
import { Servo } from '@/types/devices';

const device = defineModel<Servo>({ required: true });
</script>
