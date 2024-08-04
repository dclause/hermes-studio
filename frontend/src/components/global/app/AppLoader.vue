<template>
  <v-progress-linear
    :active="isLoading"
    :model-value="currentProgress"
    absolute
    color="deep-orange"
    height="5"
    striped
    style="top: auto; bottom: 0"
  />
</template>

<script lang="ts" setup>
import { useLoader } from '@/composables/loader';

const { isLoading, currentProgress } = useLoader();
</script>
