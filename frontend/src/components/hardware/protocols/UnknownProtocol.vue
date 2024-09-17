<template>
  <em>{{ t('unknown') }}</em>
</template>
<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { Protocol } from '@/types/boards';

defineProps<{ protocol: Protocol }>();

const { t } = useI18n();
</script>

<i18n>
{
  "en": {
    "unknown": "Unknown protocol"
  },
  "fr": {
    "unknown": "Protocole inconnu"
  }
}
</i18n>
