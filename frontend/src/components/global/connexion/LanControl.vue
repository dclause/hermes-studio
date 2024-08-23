<template>
  <v-tooltip location="bottom">
    <template #activator="{ props }">
      <v-icon
        class="mx-2 ml-3"
        :class="{ blink: status === 'pending' }"
        :icon="`mdi-lan-${status}`"
        v-bind="props"
      />
    </template>
    <span>{{ t('server') }}: {{ $t(`connexion.${status}`) }}</span>
  </v-tooltip>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useConnectionStore } from '@/stores/connectionStore';

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
    "server": "Server"
  },
  "fr": {
    "server": "Serveur"
  }
}
</i18n>
