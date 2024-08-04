<template>
  <router-link :to="to" class="app-link text-decoration-none text-primary font-weight-bold">
    <slot />
  </router-link>
</template>

<script lang="ts" setup>
import { RouterLink } from 'vue-router';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
defineProps<{ to: any }>();
</script>
