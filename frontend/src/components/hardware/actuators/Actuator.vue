<template>
  <component :is="component" v-model="device" />
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useDeviceComponent } from '@/composables/deviceComposables';
import { Actuator } from '@/types/devices';

const device = defineModel<Actuator>({ required: true });
const component = computed(() => useDeviceComponent(device.value.type));
</script>
