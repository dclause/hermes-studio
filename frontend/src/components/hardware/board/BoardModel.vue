<template>
  <component :is="component" :model="model" />
</template>
<script setup lang="ts">
import { computed } from 'vue';
import ArduinoModel from '@/components/hardware/board/ArduinoModel.vue';
import RaspberryModel from '@/components/hardware/board/RaspberryModel.vue';
import UnknownModel from '@/components/hardware/board/UnknownModel.vue';
import { BoardModel } from '@/types/hardwares';

const props = defineProps<{ model: BoardModel }>();
const component = computed(() => {
  let model;

  if (typeof props.model === 'string' || props.model instanceof String) {
    model = props.model;
  } else {
    model = Object.keys(props.model)[0];
  }
  switch (model) {
    case 'RaspberryPi':
      return RaspberryModel;
    case 'Arduino':
      return ArduinoModel;
    default:
      return UnknownModel;
  }
});
</script>
