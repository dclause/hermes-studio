<template>
  <v-tooltip location="bottom">
    <template #activator="{ props }">
      <v-btn :class="{ blink: status === 'pending' }" :icon="`mdi-lan-${status}`" v-bind="props" />
    </template>
    <span>{{ t('server') }}: {{ t(`status.${status}`) }}</span>
  </v-tooltip>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useConnectionStore } from '@/stores/connection';

const { t } = useI18n();
const store = useConnectionStore();
const { isConnected } = storeToRefs(store);
const status = computed((): string => {
  switch (isConnected.value) {
    case undefined:
      return 'pending';
    case true:
      return 'check';
    case false:
    default:
      return 'disconnect';
  }
});
</script>

<style lang="css" scoped>
@keyframes blink {
  50% {
    opacity: 0.5;
  }
}

.blink {
  animation: blink 0.5s step-start 0s infinite;
}
</style>

<i18n>
{
  "en": {
    "server": "Server",
    "status": {
      "check": "connected",
      "disconnect": "disconnected",
      "offline": "offline",
      "pending": "connecting..."
    }
  },
  "fr": {
    "server": "Serveur",
    "status": {
      "check": "connecté",
      "disconnect": "déconnecté",
      "offline": "hors ligne",
      "pending": "connexion..."
    }
  }
}
</i18n>
