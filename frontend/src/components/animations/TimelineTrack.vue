<template>
  <timeline-track-device v-if="track.device" v-model="track" class="track" />
  <timeline-track-group v-else v-model="track" class="track" />
</template>

<script setup lang="ts">
import { useTimeline } from '@/composables/timelineComposables';
import { Track } from '@/types/timeline';

const track = defineModel<Track>({ required: true });
// CSS binding.
const { config } = useTimeline();
</script>
<style lang="scss" scoped>
.track {
  border-top: 1px solid rgb(var(--v-theme-timeline-border));
  height: v-bind('config.trackHeight + "px"');
  box-sizing: border-box;
}
</style>
