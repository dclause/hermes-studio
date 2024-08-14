<template>
  <v-text-field
    v-model.number="device.pin"
    :label="t('pin')"
    type="number"
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
device.value.intensity = 255;
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
