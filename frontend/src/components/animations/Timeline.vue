<template>
  <div tabindex="0" class="timeline">
    <timeline-header class="timeline-header" :on-save="unsavedActions > 1 && saveAnimation" />
    <div class="timeline-container">
      <div ref="tracksContainer" class="tracks">
        <template v-for="(track, idx) in tracks" :key="track.id">
          <timeline-track v-model="tracks[idx]" class="track" />
        </template>
      </div>
      <div ref="timelineContainer" class="timeline" tabindex="0" />
    </div>

    <confirm-dialog
      v-model="showConfirmPopup"
      title="You have unsaved changes"
      text="You are about to leave the page. Would you like to save changes first ?"
      :confirm="$t('form.save')"
      @cancel="confirmDialogUtils.setDialogResponse(false)"
      @confirm="confirmDialogUtils.setDialogResponse(true)"
    />
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import { onBeforeRouteLeave } from 'vue-router';
import { TimelineEvents } from '@/components/animations/timeline/timeline.events';
import { DialogUtils } from '@/composables/formComposables';
import { logError } from '@/composables/globalComposables';
import { useNestedToFlat } from '@/composables/groupComposables';
import { useTimeline } from '@/composables/timelineComposables';
import { useAnimationStore } from '@/stores/animationStore';
import { useGroupStore } from '@/stores/groupStore';
import { Animation, Keyframe } from '@/types/animation';
import { FlatGroup, GroupId } from '@/types/groups';
import { Track } from '@/types/timeline';

// Build timeline.
const { timeline, config, reset, groupsToTracks } = useTimeline();
watch(config, (config) => {
  timeline.setConfig(config);
  timeline.refresh();
});

const animation = defineModel<Animation>({ required: true });
const { groups } = storeToRefs(useGroupStore());

/** Build tracks */
const tracks = ref<Track[]>(groupsToTracks(animation.value, groups.value));
watch(
  [animation, groups],
  ([animation, groups]: [Animation, Record<GroupId, FlatGroup>]) => {
    tracks.value = groupsToTracks(animation, groups);
  },
  { deep: true },
);

/** confirmPopup is used to avoid leaving the page with unsaved changes */
const showConfirmPopup = ref(false);
const confirmDialogUtils = new DialogUtils<boolean>();

const emit = defineEmits(['selectKeyframe']);

/** Elements to act on */
const tracksContainer = ref<HTMLElement>();
const timelineContainer = ref<HTMLElement>();

// Displays a warning before the user leaves the page (vue router navigation).
onBeforeRouteLeave(async (to, from, next) => {
  if (unsavedActions.value > 1) {
    showConfirmPopup.value = true;
    // Saves the animation if the confirmDialog answer is yes.
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    (await confirmDialogUtils.getDialogResponse()) && saveAnimation();
    showConfirmPopup.value = false;
  }
  next();
});

// Counts the number of unsaved actions (those are emitted by timeline saveInHistory()).
// NOTE: action 0 is always the timeline initialization, which we don't want to count as
// an unsavedAction.
const unsavedActions = ref(0);

// Displays a warning before the user leaves the page (F5).
window.onbeforeunload = (event) => {
  if (unsavedActions.value > 1) {
    event.preventDefault();
  }
};

// Create the timeline.
onMounted(() => {
  nextTick(() => {
    reset();
    timeline.init(timelineContainer.value!, tracks.value, config.value);
  });
});
onUnmounted(() => {
  timeline.destroy();
});
// Anytime the tracks change (open/close, new keyframes, etc..) we need to redraw the whole canvas.
watch(
  [tracks],
  () => {
    nextTick(() => {
      unsavedActions.value++;
      timeline.setTracks(tracks?.value);
    });
  },
  { deep: true },
);

timeline.on(TimelineEvents.updateTracks, () => unsavedActions.value++);
timeline.on(TimelineEvents.selectKeyframe, (item) => {
  emit('selectKeyframe', item);
});
timeline.on(TimelineEvents.scroll, (scrollTop) => {
  tracksContainer.value!.scrollTop = scrollTop;
});

/** Saves the animation handler. */
const animationStore = useAnimationStore();
const saveAnimation = () => {
  // Flatten the tracks.
  const flatTracks = useNestedToFlat(tracks.value) as unknown as Track[];
  // Build the keyframes array.
  animation.value.tracks = Object.values(flatTracks).reduce(
    (keyframes, track: Track) => {
      keyframes[track.id] = [...track.keyframes];
      return keyframes;
    },
    {} as Record<GroupId, Keyframe[]>,
  );
  // Save.
  animationStore
    .update(animation.value)
    .then(() => {
      unsavedActions.value = 0;
      return null;
    })
    .catch(logError);
};
</script>

<style lang="scss" scoped>
.timeline {
  background-color: rgb(var(--v-theme-surface-variant));
  color: rgb(var(--v-theme-on-primary));
  display: flex;
  flex-direction: column;
  position: relative;

  .timeline-container {
    height: calc(100% - 35px) !important;
    display: flex;
    position: relative;
    overflow: hidden;
  }

  .tracks {
    border-right: 1px solid rgb(var(--v-theme-timeline-border));
    padding: v-bind('config.headerHeight + "px"') 0 0 0;
    margin-bottom: 17px;
    overflow: hidden;
    flex: 0 0 16.666%;

    > *:last-child {
      border-bottom: 1px solid rgb(var(--v-theme-timeline-border));
    }
  }

  .timeline {
    flex: 0 0 83.333%;
  }
}
</style>
