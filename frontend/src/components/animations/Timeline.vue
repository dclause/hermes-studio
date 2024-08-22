<template>
  <div tabindex="0" class="timeline">
    <!--    <timeline-header class="timeline-header" :on-save="unsavedActions > 1 && saveAnimation" />-->
    <div class="timeline-container">
      <div ref="tracksContainer" class="v-col v-col-2 tracks">
        <template v-for="(track, idx) in tracks" :key="track.id">
          <timeline-track v-model="tracks[idx]" class="track" />
        </template>
      </div>
      <div
        ref="timelineContainer"
        class="position-relative"
        style="display: flex; flex: 1 1 100%"
        tabindex="0"
      />
    </div>

    <confirm-dialog
      v-model="showConfirmPopup"
      title="You have unsaved changes"
      text="You are about to leave the page. Would you like to save changes first ?"
      @cancel="confirmDialogUtils.setDialogResponse(false)"
      @confirm="confirmDialogUtils.setDialogResponse(true)"
    />
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { nextTick, onMounted, onUnmounted, ref, watch } from 'vue';
import { onBeforeRouteLeave } from 'vue-router';
import { DialogUtils } from '@/composables/formComposables';
import { logError } from '@/composables/globalComposables';
import { useFlatToNested } from '@/composables/groupComposables';
import { useTimeline } from '@/composables/timelineComposables';
import { useAnimationStore } from '@/stores/animationStore';
import { useGroupStore } from '@/stores/groupStore';
import { Animation, Keyframe } from '@/types/animation';
import { FlatGroup, GroupId } from '@/types/groups';
import { Track } from '@/types/timeline';

const animation = defineModel<Animation>({ required: true });
const { groups } = storeToRefs(useGroupStore());

// Retrieve tracks
const tracks = ref<Track[]>(useFlatToNested(groups.value, animation.value.keyframes));
watch(
  [animation.value.keyframes, groups],
  ([keyframes, groups]: [Record<GroupId, Keyframe[]>, Record<GroupId, FlatGroup>]) => {
    tracks.value = useFlatToNested(groups, keyframes);
  },
);

const { timeline, config, reset } = useTimeline();

/** confirmPopup is used to avoid leaving the page with unsaved changes */
const showConfirmPopup = ref(false);
const confirmDialogUtils = new DialogUtils<boolean>();

const emit = defineEmits(['selectKeyframe']);

/** Build tracks */
// const { buildTracks, flattenTracks } = useTracks();
// const tracks = ref(buildTracks(animation.value.keyframes));

/** Elements to act on */
const tracksContainer = ref<HTMLElement>();
const timelineContainer = ref<HTMLElement>();

onMounted(() => {
  nextTick(() => {
    reset();
    // timeline.initialize(timelineContainer.value!, tracks.value);
  });
});
onUnmounted(() => {
  // timeline.destroy();
});

// Displays a warning before the user leaves the page (vue router navigation).
onBeforeRouteLeave(async (to, from, next) => {
  if (unsavedActions.value > 1) {
    showConfirmPopup.value = true;
    // Saves the animation if the confirmDialog answser is yes.
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

// Anytime the tracks change (open/close, new keyframes, etc..) we need to redraw the whole canvas.
// watch(
//   [tracks],
//   () => {
//     nextTick(() => {
//       unsavedActions.value++;
//       timeline?.setTracks(tracks?.value ?? []);
//     });
//   },
//   { deep: true },
// );

// timeline.on(TimelineEvents.updateTracks, () => unsavedActions.value++);
// timeline.on(TimelineEvents.selectKeyframe, (item) => emit('selectKeyframe', item));
// timeline.on(TimelineEvents.scroll, (scrollTop) => {
//   tracksContainer.value!.scrollTop = scrollTop;
// });

/** Saves the animation handler. */
const animationStore = useAnimationStore();
const saveAnimation = () => {
  // // Flatten the tracks.
  // const flatTracks = flattenTracks(tracks.value);
  // // Build the keyframes array.
  // animation.value.keyframes = flatTracks.reduce((keyframes, track: Track) => {
  //   keyframes.push(...track.keyframes);
  //   return keyframes;
  // }, [] as Keyframe[]);
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
  }

  .tracks {
    margin: 1px;
    border-right: 1px solid rgb(var(--v-theme-timeline-border));
    padding: v-bind('config.headerHeight + "px"') 0 0 0;
    margin-bottom: 17px;
    overflow: hidden;

    > *:last-child {
      border-bottom: 1px solid rgb(var(--v-theme-timeline-border));
    }
  }
}
</style>
