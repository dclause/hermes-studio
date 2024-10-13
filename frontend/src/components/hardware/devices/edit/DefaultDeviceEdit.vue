<template>
  <v-text-field
    v-model.number="device.pin"
    type="number"
    :label="t('pin')"
    required
    :rules="[Rule.REQUIRED]"
  />
  <slot />
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';
import { Rule } from '@/composables/formComposables';
import { Device } from '@/types/devices';

const { t } = useI18n();
const device = defineModel<Device>({ required: true });
</script>

<i18n>
{
  "en": {
    "pin": "Board pin"
  },
  "fr": {
    "pin": "Pin (broche)"
  }
}
</i18n>
