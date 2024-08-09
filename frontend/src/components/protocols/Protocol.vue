<template>
  <component :is="component" :protocol="protocol" />
</template>
<script setup lang="ts">
import { computed } from 'vue';
import SerialProtocol from '@/components/protocols/SerialProtocol.vue';
import UnknownProtocol from '@/components/protocols/UnknownProtocol.vue';
import { Protocol } from '@/types/boards';

const props = defineProps<{ protocol: Protocol }>();
const component = computed(() => {
  switch (props.protocol.type) {
    case 'SerialProtocol':
      return SerialProtocol;
    default:
      return UnknownProtocol;
  }
});
</script>
