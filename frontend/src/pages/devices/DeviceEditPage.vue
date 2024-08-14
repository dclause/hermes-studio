<template>
  <v-card class="mx-auto pa-4" variant="elevated" width="600">
    <v-form ref="form" :disabled="loading" :loading="loading" @submit.prevent="onSubmit">
      <v-text-field v-model="device.name" label="Name" required :rules="[Rule.REQUIRED]" />
      <component :is="editComponent" v-model="device" />

      <!-- Submit -->
      <v-row>
        <v-col class="align-self-center" cols="12" md="6">
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
            {{ $t('form.edit') }}
          </v-btn>
        </v-col>
        <v-col class="align-self-center" cols="12" md="6">
          <v-btn
            block
            class="mt-2"
            color="surface-light"
            :disabled="loading"
            :loading="loading"
            size="large"
            variant="elevated"
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
import type { Device, DeviceId } from '@/types/devices';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { VForm } from 'vuetify/components';
import { useDeviceEditComponent } from '@/composables/deviceComposables';
import { Rule } from '@/composables/formComposables';
import { logError } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { BoardId } from '@/types/boards';

const route = useRoute();
const router = useRouter();
const id = Number(route.params.id) as DeviceId;

// Create new device.
const deviceStore = useDeviceStore();
const { devices } = storeToRefs(deviceStore);
const device = computed<Device>({
  get: () => devices.value[id],
  set: (value: Device) => {
    devices.value[id] = value;
  },
});

// Create new form.
const form = ref<VForm>();

// Update the create/edit specific device type component.
const editComponent = computed(() => useDeviceEditComponent(device.value.type));

// Save the newly created device.
const bid = computed<BoardId>(() => device.value?.bid);
const { loading } = storeToRefs(deviceStore);
const onSubmit = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    deviceStore
      .create(device.value)
      .then(() => {
        return router.push({ name: 'board.show', params: { bid: bid.value } });
      })
      .catch(logError);
  }
};

// Cancel: return to previous page
const onCancel = () => {
  return useRouter().go(-1);
};
</script>
