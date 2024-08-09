<template>
  <em>{{ t('unknown') }}</em>
</template>
<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { BoardModel } from '@/types/boards';

defineProps<{ model: BoardModel }>();

const { t } = useI18n();
</script>

<i18n>
{
  "en": {
    "unknown": "Unknown board model"
  },
  "fr": {
    "unknown": "Carte de type inconnu"
  }
}
</i18n>
