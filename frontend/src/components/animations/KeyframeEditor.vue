<template>
  <v-card class="pt-3">
    <div class="d-flex justify-space-between align-center">
      <v-card-title class="py-0">
        {{ track.name }}
      </v-card-title>
      <v-card-subtitle v-if="board" class="py-0">
        {{ board.name }}
      </v-card-subtitle>
    </div>
    <v-card-text>
      <v-row class="ma-0">
        <v-col class="align-self-center">
          <v-text-field
            v-model.number="keyframe.start"
            type="number"
            hide-details
            class="command-input flex-0-0"
            :label="t('start')"
            :min="0"
          />
        </v-col>
        <v-col class="align-self-center">
          <v-text-field
            v-model.number="keyframe.end"
            type="number"
            hide-details
            class="command-input flex-0-0"
            :label="t('end')"
            :min="0"
          />
        </v-col>
      </v-row>
      <div class="pa-3">
        <div v-for="position in keyframe.positions" :key="position.device">
          <component
            :is="useDeviceComponent(devices[position.device].type)"
            v-model="position.target"
            :device="devices[position.device] as Actuator"
            variant="minimal"
            hide-label
            :mode="HardwareMode.OFF"
          />
        </div>
      </div>
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
import { Track } from '@/types/timeline';

const { t } = useI18n();
const keyframe = defineModel<Keyframe>({ required: true });
const props = defineProps<{ track: Track }>();

const { devices } = useDeviceStore();
const { boards } = useBoardStore();
const device = computed(() => props.track.device && devices[props.track.device]);
const board = computed(() => device.value && device.value.bid && boards[device.value.bid]);

// onBeforeMount(() => {
//   const buildPositionsForGroup = (group: FlatGroup): Position[] => {
//     return group.children.reduce((positions, childGroup) => {
//       // const childGroup = groups[child];
//       if (childGroup) {
//         if (childGroup.device) {
//           const device = devices[childGroup.device];
//           if (device) {
//             positions.push({
//               device: device.id,
//               target:
//                 keyframe.value.positions?.find((position) => position.device === device.id)
//                   ?.target ?? device.default,
//             });
//           }
//         } else {
//           positions.push(...buildPositionsForGroup(childGroup));
//         }
//       }
//       return positions;
//     }, [] as Position[]);
//   };
//   keyframe.value.positions = buildPositionsForGroup(props.track);
// });
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
