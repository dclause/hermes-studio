<template>
  <div id="container" class="d-flex flex-column">
    <v-form
      v-if="posture"
      id="edition-form"
      ref="form"
      :disabled="loading"
      :loading="loading"
      class="d-flex flex-column pa-8 position-relative"
      @submit.prevent="onSave"
    >
      <h1 class="text-h5 text-md-h4">
        <v-icon icon="mdi-camera-control" />
        {{ posture.name }}
      </h1>
      <div class="ml-2 font-italic">
        {{ posture.description }}
      </div>

      <v-expansion-panels v-model="panel" class="my-5">
        <v-expansion-panel value="edit">
          <v-expansion-panel-title>{{ t('edit') }}</v-expansion-panel-title>

          <v-expansion-panel-text>
            <v-row>
              <v-text-field
                v-model="posture.name"
                :label="t('name')"
                required
                :rules="[Rule.REQUIRED]"
              />
            </v-row>
            <v-row>
              <v-textarea v-model="posture.description" :label="t('description')" />
            </v-row>
          </v-expansion-panel-text>
        </v-expansion-panel>
      </v-expansion-panels>

      <nested-group v-model="nestedGroups" variant="minimal" />

      <v-spacer class="mb-5" />

      <!-- Submit -->
      <v-row class="position-fixed sticky-save">
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
    </v-form>
  </div>
</template>
<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { VForm } from 'vuetify/components';
import { Rule } from '@/composables/formComposables';
import { logError, useRedirect } from '@/composables/globalComposables';
import { useFlatToNested } from '@/composables/groupComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { useGroupStore } from '@/stores/groupStore';
import { usePostureStore } from '@/stores/postureStore';
import { PostureId } from '@/types/postures';

const { t } = useI18n();
const { devices } = storeToRefs(useDeviceStore());

/** Retrieve the posture from the URL parameter */
const postureStore = usePostureStore();

const { redirect } = useRedirect();
const route = useRoute();
const id = Number(route.params.id) as PostureId;
const posture = computed(() => {
  const postureFromStore = postureStore.get(id);
  if (postureFromStore) {
    // Reset to this posture
    postureStore.play(postureFromStore.id);
  }
  // Force work on copy
  return { ...postureFromStore };
});

// Selected panel.
const panel = ref([]);

// Build nested view.
const groupStore = useGroupStore();
const { groups } = storeToRefs(groupStore);
const nestedGroups = computed(() => {
  return useFlatToNested(groups.value);
});

const onCancel = () => {
  panel.value = [];
  redirect();
};

// Create new form.
const form = ref<VForm>();

// Save the posture.
const loading = ref<boolean>(false);
const onSave = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    loading.value = true;

    posture.value.positions = Object.values(devices.value).map((device) => {
      return { device: device.id, target: device.state };
    });

    postureStore
      .update(posture.value)
      .then(() => {
        panel.value = [];
        return redirect('posture.list', true);
      })
      .catch(logError);
    loading.value = false;
  }
};
</script>

<style lang="scss" scoped>
.sticky-save {
  background-color: rgba(var(--v-theme-surface), 0.5) !important;
  left: var(--v-layout-left);
  bottom: 0;
  width: calc(100% - var(--v-layout-left));
  margin: 0;
  padding-inline: 20em;
}
</style>
<i18n>
{
  "en": {
    "edit": "Edit posture settings",
    "name": "Name",
    "description": "Description"
  },
  "fr": {
    "edit": "Modifier les configuration de la posture",
    "name": "Nom",
    "description": "Description"
  }
}
</i18n>
