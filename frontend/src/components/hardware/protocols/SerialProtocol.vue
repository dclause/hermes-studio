<template>
  <div>
    {{ t('label', { port: protocol.port }) }}
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
    "label": "Serial protocol on {port}"
  },
  "fr": {
    "label": "Protocole série sur {port}"
  }
}
</i18n>
