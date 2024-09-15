<template>
  <div class="controls">
    <v-btn
      class="btn"
      color="white"
      icon="mdi-restart"
      variant="text"
      size="x-small"
      @click="restart"
    />
    <v-btn
      class="btn"
      color="white"
      icon="mdi-skip-previous"
      variant="text"
      size="x-small"
      @click="previousFrame"
    />
    <v-btn
      class="btn"
      color="white"
      icon=""
      variant="text"
      size="x-small"
      @click="toggle"
    >
      <template #default>
        <v-icon v-if="activeRafId" icon="mdi-pause" />
        <v-icon v-else icon="mdi-play" />
      </template>
    </v-btn>
    <v-btn
      class="btn"
      color="white"
      icon="mdi-skip-next"
      variant="text"
      size="x-small"
      @click="nextFrame"
    />
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { ref } from 'vue';
import { HardwareMode, logError } from '@/composables/globalComposables';
import { useNestedToFlat } from '@/composables/groupComposables';
import { useTimeline } from '@/composables/timelineComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useConfigStore } from '@/stores/configurationStore';
import { useDeviceStore } from '@/stores/deviceStore';
import { Keyframe } from '@/types/animation';
import { Track } from '@/types/timeline';

const { timeline } = useTimeline();
const boardStore = useBoardStore();
const deviceStore = useDeviceStore();

let activeRafId = ref<number | null>(null);

/**
 * Defines the ref holders for time management when press (play).
 */
let startTimestamp = Date.now();
let currentTimestamp = startTimestamp;

/**
 * Toggles the animation: stores the current timestamp
 * when the animation is either started or paused.
 */
const restart = () => {
  startTimestamp = Date.now();
  currentTimestamp = startTimestamp;
  pause();
  timeline?.setTime(0);
  boardStore.reset_all();
};

/**
 * Toggles the animation: stores the current timestamp
 * when the animation is either started or paused.
 */
const toggle = () => {
  if (activeRafId.value) {
    pause();
  } else {
    startTimestamp += Date.now() - currentTimestamp;
    resume();
  }
};

/**
 * Previous frame: 500ms before.
 */
const previousFrame = () => {
  startTimestamp = Math.min(currentTimestamp, startTimestamp + 500);
  timeline?.setTime(currentTimestamp - startTimestamp);
};

/**
 * Previous frame: 500ms after.
 */
const nextFrame = () => {
  startTimestamp = Math.max(0, startTimestamp - 500);
  timeline?.setTime(currentTimestamp - startTimestamp);
};

const resume = () => {
  if (!activeRafId.value) {
    // Flatten the tracks.
    const flatTracks = useNestedToFlat<Track, Track>(timeline.getTracks());
    // Build an array of keyframes for active tracks only, with data for connected board only.
    // This is done before the actual start to maximize the efficiency of the realtime "demo" mode.
    activatedKeyFrames = Object.values(flatTracks).reduce((keyframes, track: Track) => {
      // Remove disabled tracks.
      if (!track.disabled && track.keyframes.length) {
        // const device = deviceStore.get(track.device);
        // const board = boardStore.get(device.bid);
        // if (board.connected) {
        // Remove keyframes for none connected boards.
        keyframes.push(...track.keyframes);
        // }
      }
      return keyframes;
    }, [] as Keyframe[]);

    activeRafId.value = window.requestAnimationFrame(loop);
  }
};

const pause = () => {
  activeRafId.value = null;
};

const loop = () => {
  currentTimestamp = Date.now();
  const elapsed = currentTimestamp - startTimestamp;
  if (elapsed > (timeline?.getDuration() ?? 0)) {
    pause();
  } else {
    timeline?.setTime(elapsed);
    playKeyframeAt(elapsed);
  }

  if (activeRafId.value) {
    activeRafId.value = window.requestAnimationFrame(loop);
  }
};

// #####
// Playing keyFrames.

const { mode } = storeToRefs(useConfigStore());
// const deviceStore = useDeviceStore();
let lastPlayedTime: number = -1;
let activatedKeyFrames: Keyframe[] = [];

/** Play the keyframe at given time. In practice, we are playing the next available keyframe  */
const playKeyframeAt = (time: number) => {
  if (mode.value === HardwareMode.VIRTUAL) {
    lastPlayedTime = -1;
    return;
  }

  // playableKeyframes are the ones between the last one played and all the ones to be played in the next 50ms.
  const playableKeyframe = activatedKeyFrames.toReversed().filter((kf) => {
    return kf.start > lastPlayedTime && kf.start < time + 50;
  });
  for (const nextKeyFrame of playableKeyframe) {
    for (const position of nextKeyFrame.positions) {
      deviceStore
        .animate(
          position.device,
          position.target,
          nextKeyFrame.end - nextKeyFrame.start,
          nextKeyFrame.transition,
        )
        .catch(logError);
    }
  }

  lastPlayedTime = time + 50;
};
</script>

<style lang="scss" scoped>
.controls .btn {
  font: inherit;
}
</style>
