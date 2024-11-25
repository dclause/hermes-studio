<template>
  <div>
    {{ label }}
  </div>
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { Protocol } from '@/types/hardwares';

const { t } = useI18n();
const props = defineProps<{ protocol: Protocol }>();
const label = computed(() => {
  const transport = props.protocol.transport!;
  switch (transport.type) {
    case 'Serial':
      return t('serial', { port: transport.port });
    default:
      return transport.type;
  }
});
</script>

<i18n>
{
  "en": {
    "serial": "Serial protocol on {port}"
  },
  "fr": {
    "serial": "Protocole série sur {port}"
  }
}
</i18n>
