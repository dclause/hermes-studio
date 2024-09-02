<template>
  <v-form
    v-if="animation"
    ref="form"
    :disabled="loading"
    :loading="loading"
    class="d-flex flex-column"
    style="min-height: 100%"
    @submit.prevent="onSave"
  >
    <h1 class="text-h5 text-md-h4">
      <v-icon icon="mdi-movie-open" />
      {{ animation.name }}
    </h1>
    <div class="ml-2 font-italic">
      {{ animation.description }}
    </div>

    <v-expansion-panels v-model="panel" class="mt-5">
      <v-expansion-panel value="edit">
        <v-expansion-panel-title>{{ t('edit') }}</v-expansion-panel-title>

        <v-expansion-panel-text>
          <v-row>
            <v-text-field
              v-model="animation.name"
              :label="t('name')"
              required
              :rules="[Rule.REQUIRED]"
            />
          </v-row>
          <v-row>
            <v-textarea v-model="animation.description" :label="t('description')" />
          </v-row>
          <v-row>
            <v-col class="align-self-center pa-0">
              <v-text-field
                v-model="animation.fps"
                :label="t('fps')"
                :min="0"
                :max="100"
                required
                :rules="[Rule.REQUIRED]"
                type="number"
              />
            </v-col>
            <v-col class="align-self-center pa-0">
              <v-slider
                v-model="animation.speed"
                :label="t('speed')"
                :min="0"
                :max="500"
                required
                :rules="[Rule.REQUIRED]"
                thumb-label
                step="1"
              >
                <template #append>
                  <v-text-field
                    v-model="animation.speed"
                    density="compact"
                    style="width: 100px"
                    type="number"
                    variant="outlined"
                    hide-details
                    suffix="%"
                  />
                </template>
              </v-slider>
            </v-col>
          </v-row>

          <v-row>
            <v-col class="align-self-center py-2">
              <v-checkbox v-model="animation.repeat" :label="t('repeat')" />
            </v-col>
            <v-col class="align-self-center pa-0">
              <v-text-field
                v-if="animation.repeat"
                v-model="animation.loopback"
                :label="t('loopback')"
                :min="0"
                required
                :rules="[Rule.REQUIRED]"
                type="number"
              />
            </v-col>
          </v-row>

          <!-- Submit -->
          <v-row class="align-self-end">
            <v-col class="align-self-center" cols="12" sm="6">
              <v-btn
                block
                class="mt-2"
                color="primary"
                size="large"
                type="submit"
                variant="elevated"
              >
                {{ $t('form.save') }}
              </v-btn>
            </v-col>
            <v-col class="align-self-center" cols="12" sm="6">
              <v-btn block class="mt-2" size="large" variant="text" @click="onCancel">
                {{ $t('form.cancel') }}
              </v-btn>
            </v-col>
          </v-row>
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <div class="d-flex flex-grow-1">
      <keyframe-editor v-if="selectedKeyframe" v-model="selectedKeyframe" class="mt-5 w-100" />
    </div>

    <timeline
      id="timeline"
      v-model="animation"
      class="align-content-end"
      @select-keyframe="onSelectedItem"
    />
  </v-form>
</template>
<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { VForm } from 'vuetify/components';
import { Rule } from '@/composables/formComposables';
import { logError, useRedirect } from '@/composables/globalComposables';
import { useAnimationStore } from '@/stores/animationStore';
import { AnimationId, Keyframe } from '@/types/animation';

const { t } = useI18n();
const { redirect } = useRedirect();

/** Retrieve the animation from the URL parameter */
const animationStore = useAnimationStore();
const route = useRoute();
const id = Number(route.params.id) as AnimationId;
const animation = computed(() => {
  // Force work on copy
  return { ...animationStore.get(id) };
});

// Selected panel.
const panel = ref([]);

const onCancel = () => {
  panel.value = [];
  redirect();
};

// Create new form.
const form = ref<VForm>();

// Save the animation.
const loading = ref<boolean>(false);
const onSave = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    loading.value = true;
    animationStore
      .update(animation.value)
      .then(() => {
        panel.value = [];
        return;
      })
      .catch(logError);
    loading.value = false;
  }
};

/** The keyframe selected inside the animation timeline (used for edition) */
const selectedKeyframe = ref<Keyframe>();
const onSelectedItem = (item: Keyframe) => {
  selectedKeyframe.value = item;
};
</script>

<style lang="scss" scoped>
#timeline {
  width: calc(100% + 64px);
  max-height: 50vh;
  min-height: 0;
  overflow: hidden;
  left: -32px;
  bottom: -32px;
}
</style>

<i18n>
{
  "en": {
    "edit": "Edit animation settings",
    "name": "Name",
    "description": "Description",
    "repeat": "Play the animation in a loop",
    "loopback": "Timecode to restart the loop (in ms)",
    "fps": "FPS",
    "speed": "Speed of the animation (in percent)"
  },
  "fr": {
    "edit": "Modifier les configuration de l'animation",
    "name": "Nom",
    "description": "Description",
    "repeat": "Jouer l'animation en boucle",
    "loopback": "Temps (en ms) pour redémarrer la boucle",
    "fps": "FPS",
    "speed": "Vitesse d'animation (en pourcentage)"
  }
}
</i18n>
