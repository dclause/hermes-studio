<template>
  <router-view v-if="isConnected" />
  <lan-broken v-else />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { useConnectionStore } from '@/stores/connectionStore';

const store = useConnectionStore();
const { isConnected } = storeToRefs(store);
</script>
