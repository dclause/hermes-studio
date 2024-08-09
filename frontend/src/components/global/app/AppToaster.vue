<template>
  <v-snackbar
    v-for="(toast, idx) in toasts"
    :key="toast.id"
    v-model="toasts[idx].model"
    :class="'v-toaster v-toast-' + toast.id"
    :color="toast.status"
    :style="getPositionAsStyles(idx)"
    :timeout="toast.timeout"
    multi-line
    transition="fade-transition"
  >
    <v-layout class="text-center overflow-visible align-center pr-4">
      <v-icon class="pr-3 vertical" size="36">
        ${{ toast.status }}
      </v-icon>
      <v-layout column>
        <div>{{ toast.text }}</div>
      </v-layout>
    </v-layout>
    <template #actions>
      <v-btn icon="mdi-close" variant="plain" @click="toasterStore.remove(toast.id)" />
    </template>
  </v-snackbar>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { useToasterStore } from '@/stores/toastStore';

const toasterStore = useToasterStore();
const { toasts } = storeToRefs(toasterStore);

const getPositionAsStyles = (idx: number) => {
  return {
    transition: 'all 0.2s',
    transform: `translateY(-${idx * 5}px) scale(${1 - idx * 0.03})`,
  };
};
</script>
