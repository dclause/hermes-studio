<template>
  <default-expander :expander="expander" class="expander-pca9685" :mode="mode" :variant="variant">
    <template #prefix>
      <slot name="prefix" />
    </template>

    <template #icon>
      <v-icon icon="mdi-chip" class="ml-2 mr-3" size="35" />
    </template>

    <template #info>
      <div class="font-weight-bold">
        {{ $t('entities.expander') }} PCA9685
      </div>
      <div>{{ t('address') }} {{ '0x' + expander.address.toString(16) }}</div>
      <div>{{ t('frequency') }} {{ expander.frequency }}</div>
    </template>
  </default-expander>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { PCA9685 } from '@/types/hardwares';

const { t } = useI18n();
withDefaults(
  defineProps<{
    expander: PCA9685;
    mode?: HardwareMode;
    variant?: CommandMode;
  }>(),
  { mode: HardwareMode.REALTIME, variant: CommandMode.FULL },
);
</script>

<i18n>
{
  "en": {
    "address": "Address:",
    "frequency": "Frequency:"
  },
  "fr": {
    "address": "Adresse :",
    "frequency": "Fréquence :"
  }
}
</i18n>
