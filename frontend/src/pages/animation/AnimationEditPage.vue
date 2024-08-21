<template>
  <div class="d-flex flex-column">
    <h1 class="text-h5 text-md-h4">
      <v-icon icon="mdi-movie-open" />
      {{ animation.name }}
    </h1>
    <div class="ml-2 font-italic">
      {{ animation.description }}
    </div>
  </div>

  <!--  <keyframe-editor v-if="selectedKeyframe" v-model="selectedKeyframe" />-->
  <!--  <timeline id="animaker" v-model="animation" @select-keyframe="onSelectedItem" />-->
</template>
<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import { useAnimationStore } from '@/stores/animationStore';
import { AnimationId } from '@/types/animation';

/** Retrieve the animation from the URL parameter */
const animationStore = useAnimationStore();
const route = useRoute();
const id = Number(route.params.id) as AnimationId;
const animation = computed(() => animationStore.get(id));
</script>
