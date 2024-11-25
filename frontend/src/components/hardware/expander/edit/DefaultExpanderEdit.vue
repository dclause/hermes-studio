<template>
  <slot />
</template>

<script lang="ts" setup>
import { Expander } from '@/types/hardwares';

defineModel<Expander>({ required: true });
</script>
