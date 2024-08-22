<template>
  <div
    class="group"
    :class="{
      expanded: track.open,
      closed: !track.open,
    }"
  >
    <div
      class="track indent"
      :class="{
        expandable: !!track.children.length,
      }"
    >
      <v-btn
        v-if="track.children.length"
        :block="true"
        class="button"
        rounded="0"
        variant="text"
        @click="onToggleOpen"
      >
        <template #prepend>
          <v-icon v-if="track.open" icon="mdi-chevron-down" />
          <v-icon v-else icon="mdi-chevron-right" />
        </template>
        {{ track.name ?? track?.name }}
      </v-btn>
      <div v-else class="button text-button pl-4">
        {{ track.name ?? track?.name }}
      </div>
    </div>
    <div v-if="track.open" class="tracks">
      <template v-for="(child, idx) in track.children" :key="child.id">
        <timeline-track v-model="track.children[idx]" />
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTimeline } from '@/composables/timelineComposables';
import { Track } from '@/types/timeline';

const track = defineModel<Track>({ required: true });
const { config } = useTimeline();

const onToggleOpen = () => {
  track.value.open = !track.value.open;
};
</script>

<style lang="scss" scoped>
.group {
  &.expanded {
    display: block !important;
    height: auto !important;
    border: none !important;

    .track {
      border-top: 1px solid #737070;
    }
  }

  .track {
    height: v-bind('config.trackHeight + "px"');
    display: flex;
    align-items: center;
    overflow: hidden;
  }

  .indent {
    padding-left: v-bind('(track.level * 40) + "px"');
  }

  .button {
    justify-content: flex-start;
  }
}
</style>
