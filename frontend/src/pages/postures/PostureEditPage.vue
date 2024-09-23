<template>
  <div id="container" class="d-flex flex-column">
    <v-form
      v-if="posture"
      id="edition-form"
      ref="form"
      :disabled="loading"
      :loading="loading"
      class="d-flex flex-column pa-8"
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

      <div v-if="Object.values(devices).length">
        <div v-for="device in devices" :key="device.id">
          <component
            :is="useDeviceComponent(device.type)"
            v-model="device.state"
            class="pr-4"
            :device="device as Actuator"
            variant="minimal"
          />
        </div>
      </div>

      <!-- Submit -->
      <v-row class="my-5 mx-10">
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
import type { Actuator } from '@/types/devices';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRoute } from 'vue-router';
import { VForm } from 'vuetify/components';
import { useDeviceComponent } from '@/composables/deviceComposables';
import { Rule } from '@/composables/formComposables';
import { logError, useRedirect } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
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
  const newPosture = postureStore.get(id);
  if (newPosture) {
    // Reset to this posture
    postureStore.play(newPosture);
  }
  // Force work on copy
  return { ...newPosture };
});

// Selected panel.
const panel = ref([]);

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
        return redirect();
      })
      .catch(logError);
    loading.value = false;
  }
};
</script>

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
