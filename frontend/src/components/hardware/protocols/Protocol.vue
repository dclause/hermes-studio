<template>
  <component :is="component" :protocol="protocol" />
</template>
<script setup lang="ts">
import { computed } from 'vue';
import RaspiProtocol from '@/components/hardware/protocols/RaspiProtocol.vue';
import SerialProtocol from '@/components/hardware/protocols/SerialProtocol.vue';
import UnknownProtocol from '@/components/hardware/protocols/UnknownProtocol.vue';
import { Protocol } from '@/types/boards';

const props = defineProps<{ protocol: Protocol }>();
const component = computed(() => {
  switch (props.protocol.type) {
    case 'RaspiProtocol':
      return RaspiProtocol;
    case 'SerialProtocol':
      return SerialProtocol;
    default:
      return UnknownProtocol;
  }
});
</script>
