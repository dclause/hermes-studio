<template>
  <v-card class="mx-auto pa-4" variant="elevated" width="400">
    <v-form ref="form" :disabled="loading" :loading="loading" @submit.prevent="onSubmit">
      <v-text-field v-model="device.name" label="Name" required :rules="[Rule.REQUIRED]" />

      <v-select
        v-model="device.bid"
        :items="boardItems"
        label="Board"
        required
        item-title="name"
        item-value="id"
      />

      <v-select
        v-model="device.type"
        :items="['LedDevice', 'ServoDevice']"
        label="Device type"
        required
        @update:model-value="onChangeDeviceType"
      />

      <!-- Submit -->
      <v-btn
        block
        class="mt-2"
        color="primary"
        :disabled="!device.type || loading"
        :loading="loading"
        size="large"
        type="submit"
        variant="elevated"
      >
        Create device
      </v-btn>
      <v-btn
        block
        class="mt-2"
        color="secondary"
        :disabled="loading"
        :loading="loading"
        size="large"
        variant="elevated"
        @click="onCancel"
      >
        Cancel
      </v-btn>
    </v-form>
  </v-card>
</template>

<script lang="ts" setup>
import type { BoardId } from '@/types/boards';
import type { Actuator } from '@/types/devices';
import { storeToRefs } from 'pinia';
import { computed, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { VForm } from 'vuetify/components';
import { Rule } from '@/composables/formComposables';
import { logError } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';

const route = useRoute();
const bid = Number(route.params.bid) as BoardId;

// Create new device.
const deviceStore = useDeviceStore();
const device = ref<Actuator>(deviceStore.default());

// Build the board selection.
const { boards } = storeToRefs(useBoardStore());
const boardItems = computed(() => Object.values(boards.value));
// Set first available board as default if only one is available.
watch(boardItems, (newBoardsData) => {
  if (!device.value.bid && newBoardsData.length === 1) {
    device.value.bid = newBoardsData[0].id;
  }
});

// Create new form.
const form = ref<VForm>();

const onChangeDeviceType = (type: string) => {};

// Save the newly created device.
const { loading } = storeToRefs(deviceStore);
const onSubmit = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    deviceStore
      .create(device.value)
      .then(async () => {
        return useRouter().push({ name: 'board.show', params: { bid } });
      })
      .catch(logError);
  }
};

// Cancel: return to previous page
const onCancel = () => {
  return useRouter().go(-1);
};
</script>
