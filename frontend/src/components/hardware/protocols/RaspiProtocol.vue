<template>
  <div>
    {{ t('label') }}
  </div>
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
    "label": "Raspi protocol"
  },
  "fr": {
    "label": "Protocole Raspi"
  }
}
</i18n>
