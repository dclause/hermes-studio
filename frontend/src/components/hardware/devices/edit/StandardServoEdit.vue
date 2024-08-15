<template>
  <default-device-edit v-model="device">
    <v-row>
      <v-col class="align-self-center">
        <v-select
          v-model="device.degree_range[1]"
          hide-details
          :items="[
            { value: 180, title: '180°' },
            { value: 360, title: '360°' },
          ]"
          :label="t('type')"
          required
          :rules="[Rule.REQUIRED]"
        />
      </v-col>
    </v-row>

    <v-row class="mb-0">
      <v-col>
        <v-label>
          {{ t('restriction', { min: device.range[0], max: device.range[1] }) }}
        </v-label>
        <v-range-slider
          v-model="device.range"
          :min="device.degree_range[0]"
          :max="device.degree_range[1]"
          step="1"
          hide-details
          strict
          :thumb-label="true"
        >
          <template #thumb-label="{ modelValue }">
            <div>{{ modelValue }}°</div>
          </template>
          <template #prepend>
            <div class="text-caption text-center">
              min: {{ device.degree_range[0] }}°
            </div>
          </template>
          <template #append>
            <div class="text-caption text-center">
              max: {{ device.degree_range[1] }}°
            </div>
          </template>
        </v-range-slider>
      </v-col>
    </v-row>

    <v-row class="mu-0 mb-6">
      <v-col class="py-0">
        <v-label>{{ t('default', { default: device.default }) }}</v-label>
        <v-slider
          v-model="device.default"
          :min="device.range[0]"
          :max="device.range[1]"
          step="1"
          hide-details
          :thumb-label="true"
        >
          <template #prepend>
            <div class="text-caption text-center">
              min: {{ device.range[0] }}°
            </div>
          </template>
          <template #append>
            <div class="text-caption text-center">
              max: {{ device.range[1] }}°
            </div>
          </template>
          <template #thumb-label="{ modelValue }">
            <div>{{ modelValue }}°</div>
          </template>
        </v-slider>
      </v-col>
    </v-row>
  </default-device-edit>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';
import { Rule } from '@/composables/formComposables';
import { Servo } from '@/types/devices';

const { t } = useI18n();

const device = defineModel<Servo>({ required: true });
// Init default values if not.
device.value.servo_type = device.value.servo_type ?? 'Standard';
device.value.range = device.value.range ?? [0, 180];
device.value.degree_range = device.value.degree_range ?? [0, 180];
device.value.pwm_range = device.value.pwm_range ?? [600, 2400];
</script>

<i18n>
{
  "en": {
    "type": "Servo type",
    "restriction": "Servo restrictions: between {min}° and {max}°",
    "default": "Default position: {default}°"
  },
  "fr": {
    "type": "Type de servo",
    "restriction": "Restriction du servo: entre {min}° et {max}°",
    "default": "Position par défaut: {default}°"
  }
}
</i18n>
