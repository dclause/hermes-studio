<template>
  <v-card class="mx-auto pa-4" variant="elevated" max-width="600" width="100%">
    <v-form ref="form" :disabled="loading" :loading="loading" @submit.prevent="onSubmit">
      <v-text-field v-model="device.name" label="Name" required :rules="[Rule.REQUIRED]" />

      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-select
            v-model="device.bid"
            :items="boardItems"
            label="Board"
            required
            :rules="[Rule.REQUIRED]"
            item-title="name"
            item-value="id"
            :disabled="isEdit"
          />
        </v-col>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-select
            v-model="device.type"
            :items="Object.keys(DeviceType).filter((type) => type !== 'Unknown')"
            label="Device type"
            required
            :rules="[Rule.REQUIRED, (value: DeviceType) => value != DeviceType.Unknown]"
            :disabled="isEdit"
          />
        </v-col>
      </v-row>
      <component :is="editComponent" v-model="device" />

      <!-- Submit -->
      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
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
            {{ $t(isEdit ? 'form.save' : 'form.create') }}
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

<script lang="ts" setup>
import type { BoardId } from '@/types/boards';
import type { Device, DeviceId } from '@/types/devices';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useRoute } from 'vue-router';
import { VForm } from 'vuetify/components';
import { DeviceType, useDeviceEditComponent } from '@/composables/deviceComposables';
import { Rule } from '@/composables/formComposables';
import { logError, useRedirect } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';

const route = useRoute();
const { redirect } = useRedirect();
const bid = route.query['board'] ? (Number(route.query['board']) as BoardId) : null;
const isEdit = route.name === 'device.edit';

// Create new device.
const deviceStore = useDeviceStore();
const device = ref<Device>(
  isEdit
    ? { ...deviceStore.devices[Number(route.params.id) as DeviceId] }
    : deviceStore.default(bid),
);

// Build the board selection.
const { boards } = storeToRefs(useBoardStore());
const boardItems = computed(() => Object.values(boards.value));

// Create new form.
const form = ref<VForm>();

// Update the create/edit specific device type component.
const editComponent = computed(() => useDeviceEditComponent(device.value.type));

// Save the newly created device.
const loading = ref<boolean>(false);
const onSubmit = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    loading.value = true;
    isEdit
      ? deviceStore
          .update(device.value)
          .then(() => redirect())
          .catch(logError)
      : deviceStore
          .create(device.value)
          .then(() => redirect())
          .catch(logError);
    loading.value = false;
  }
};

// Cancel: return to previous page
const onCancel = () => {
  return redirect();
};
</script>
