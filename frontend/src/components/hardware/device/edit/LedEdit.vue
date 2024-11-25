<template>
  <default-device-edit v-model="device" />
</template>

<script lang="ts" setup>
import { Led } from '@/types/devices';

const device = defineModel<Led>({ required: true });
device.value.brightness = 255;
</script>
