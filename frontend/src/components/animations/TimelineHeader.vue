<template>
  <div
    class="v-row"
    :class="{
      'bg-primary': mode === HardwareMode.REALTIME,
      'bg-secondary': mode === HardwareMode.VIRTUAL,
    }"
  >
    <div class="v-col v-col-2 text-center">
      <v-btn
        :disabled="!onSave"
        color="white"
        icon="mdi-content-save"
        variant="text"
        size="x-small"
        @click="onSave"
      />
    </div>
    <div class="v-col text-center position-relative">
      <div class="timer primary">
        {{ timer }}
      </div>
      <timeline-controls />
    </div>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { HardwareMode } from '@/composables/globalComposables';
import { useTimeline } from '@/composables/timelineComposables';
import { useConfigStore } from '@/stores/configurationStore';
import { useConnectionStore } from '@/stores/connectionStore';

defineProps<{
  onSave: (() => void) | false;
}>();

const server = useConnectionStore();
const { timeline } = useTimeline();
const timer = ref<string>('00:00:00:000');

const { mode } = storeToRefs(useConfigStore());
const onSwitchMode = () => {
  if (mode.value === HardwareMode.REALTIME) {
    mode.value = HardwareMode.VIRTUAL;
  } else {
    mode.value = HardwareMode.REALTIME;
  }
};

let lastTime = 0;
// timeline.on(TimelineEvents.updateTime, (time) => {
//   const duration = time;
//   const milliseconds = (duration % 1000).toString().padStart(3, '0');
//   const seconds = Math.floor((duration / 1000) % 60)
//     .toString()
//     .padStart(2, '0');
//   const minutes = Math.floor((duration / (1000 * 60)) % 60)
//     .toString()
//     .padStart(2, '0');
//   const hours = Math.floor((duration / (1000 * 60 * 60)) % 24)
//     .toString()
//     .padStart(2, '0');
//
//   timer.value = `${hours}:${minutes}:${seconds}:${milliseconds}`;
//
//   const fps = Math.floor(1000 / (time - lastTime));
//   if (fps > 0) {
//     timer.value += ` (${fps}fps)`;
//   }
//
//   lastTime = time;
// });
</script>

<style lang="scss" scoped>
@import 'vuetify/styles';

.timeline-header {
  background-color: rgb(var(--v-theme-primary));
  align-items: center;
  min-height: 35px;
  max-height: 35px;
  overflow: hidden;
  flex-wrap: nowrap;

  .timer {
    position: absolute;
    top: 50%;
    left: 1em;
    transform: translateY(-50%);
  }
}
</style>
