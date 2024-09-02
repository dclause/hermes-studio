<template>
  <v-card class="py-3">
    <div class="d-flex justify-space-between align-center">
      <v-card-title v-if="device" class="py-0">
        {{ device.name }}
      </v-card-title>
      <v-card-subtitle v-if="board" class="py-0">
        {{ board.name }}
      </v-card-subtitle>
    </div>
    <v-card-text>
      <v-row>
        <v-col class="align-self-center">
          <v-text-field
            v-model.number="keyframe.start"
            class="command-input flex-0-0"
            :label="t('start')"
            :min="0"
            type="number"
          />
        </v-col>
        <v-col class="align-self-center">
          <v-text-field
            v-model.number="keyframe.end"
            class="command-input flex-0-0"
            :label="t('end')"
            :min="0"
            type="number"
          />
        </v-col>
      </v-row>
      <v-row>
        <component
          :is="useDeviceComponent(devices[keyframe.device].type)"
          v-if="device"
          v-model="devices[keyframe.device] as Actuator"
          v-model:keyframe="keyframe"
          variant="minimal"
          :mode="HardwareMode.OFF"
        />
      </v-row>
    </v-card-text>
  </v-card>
</template>
<script setup lang="ts">
import type { Actuator } from '@/types/devices';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { useDeviceComponent } from '@/composables/deviceComposables';
import { HardwareMode } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';
import { Keyframe } from '@/types/animation';

const { t } = useI18n();
const keyframe = defineModel<Keyframe>({ required: true });
const { devices } = useDeviceStore();
const { boards } = useBoardStore();
const device = computed(() => devices[keyframe.value.device]);
const board = computed(() => boards[device.value.bid]);
</script>

<i18n>
{
  "en": {
    "start": "Keyframe start time (ms)",
    "end": "Keyframe end time (ms)"
  },
  "fr": {
    "start": "Début du keyframe (en ms)",
    "end": "Fin du keyframe (en ms)"
  }
}
</i18n>
