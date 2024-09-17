<template>
  <div>RaspberryPi {{ type }}</div>
</template>
<script setup lang="ts">
import { BoardModel } from '@/types/boards';

const props = defineProps<{ model: BoardModel }>();
const type = (props.model as { [key: string]: string })['RaspberryPi'];
</script>
