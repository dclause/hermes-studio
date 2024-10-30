<template>
  <component :is="component" :protocol="protocol" />
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useProtocolComponent } from '@/composables/boardComposables';
import { Protocol } from '@/types/boards';

const props = defineProps<{ protocol: Protocol }>();
const component = computed(() => useProtocolComponent(props.protocol.type));
</script>
