<template>
  <default-expander-edit v-model="expander">
    <v-text-field
      v-model="hexAddress"
      label="I2C Address (default 0x40)"
      placeholder="0x40"
      :rules="[validateHex]"
      outlined
      clearable
      @input="onInput"
      @blur="formatHex"
    />
  </default-expander-edit>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { PCA9685 } from '@/types/hardwares';

const expander = defineModel<PCA9685>({ required: true });
expander.value.address = expander.value.address ?? 0x40;
expander.value.frequency = 50;

const hexAddress = ref<string>('0x' + expander.value.address.toString(16));

// Validation rule for hex value input
const validateHex = (hex: string) => {
  const value = parseInt(hex, 16);
  return (
    (value >= 0x40 && value <= 0x7f && /^0x[0-9A-Fa-f]{2}$/.test(hex)) ||
    'Invalid address value: must be an hex value between 0x40 and 0x7F'
  );
};

// Format the hex value (e.g., adds "#" if missing)
const formatHex = () => {
  if (hexAddress.value && !hexAddress.value.startsWith('0x')) {
    hexAddress.value = `0x${hexAddress.value}`;
  }
};

const onInput = (value: string) => {
  hexAddress.value = value.replace(/[^x0-9A-Fa-f]/g, '').toUpperCase();
  expander.value.address = parseInt(hexAddress.value, 16);
};
</script>
