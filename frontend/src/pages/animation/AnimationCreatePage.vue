<template>
  <v-card class="mx-auto pa-4" variant="elevated" max-width="600" width="100%">
    <v-form ref="form" :disabled="loading" :loading="loading" @submit.prevent="onSubmit">
      <v-text-field v-model="animation.name" label="Name" required :rules="[Rule.REQUIRED]" />

      <v-textarea v-model="animation.description" label="Description" />

      <!-- Submit -->
      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-btn
            block
            class="mt-2"
            color="primary"
            :disabled="loading"
            :loading="loading"
            size="large"
            type="submit"
            variant="elevated"
          >
            {{ $t('form.create') }}
          </v-btn>
        </v-col>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-btn
            block
            class="mt-2"
            :disabled="loading"
            :loading="loading"
            size="large"
            variant="text"
            @click="onCancel"
          >
            {{ $t('form.cancel') }}
          </v-btn>
        </v-col>
      </v-row>
    </v-form>
  </v-card>
</template>
<script setup lang="ts">
import type { Animation } from '@/types/animation';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { VForm } from 'vuetify/components';
import { Rule } from '@/composables/formComposables';
import { logError } from '@/composables/globalComposables';
import { useAnimationStore } from '@/stores/animationStore';
import { SocketAck } from '@/types/socket';

const router = useRouter();

/** Retrieve the animation from the URL parameter */
const animationStore = useAnimationStore();
const animation = ref(animationStore.default());

// Create new form.
const form = ref<VForm>();

// Save the newly created device.
const loading = ref<boolean>(false);
const onSubmit = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    loading.value = true;
    animationStore
      .create(animation.value)
      .then((ack: SocketAck) => {
        if (ack.success) {
          const createdAnimation = ack.success as Animation;
          router.push({
            name: 'animation.edit',
            params: { id: createdAnimation.id },
          });
        }
        return;
      })
      .catch(logError);
    loading.value = false;
  }
};

// Cancel: return to previous page
const onCancel = () => {
  return router.push({ name: 'animation.list' });
};
</script>
