<template>
  <v-card class="mx-auto pa-4" variant="elevated" max-width="600" width="100%">
    <v-form ref="form" :disabled="loading || !device" :loading="loading" @submit.prevent="onSubmit">
      <v-text-field v-model="device.name" label="Name" required :rules="[Rule.REQUIRED]" />

      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-select
            v-model="device.hid"
            :items="hardwareItems"
            item-title="name"
            item-value="value"
            label="Board or Expander"
            required
            :rules="[Rule.REQUIRED]"
            :disabled="isEdit"
          >
            <template #item="{ props, item }">
              <v-list-subheader v-if="'group' in item.raw" class="group-item">
                {{ $t(item.raw.group) }}s
              </v-list-subheader>
              <v-list-item v-else v-bind="props" />
            </template>
          </v-select>
        </v-col>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-select
            v-model="device.type"
            :items="mapEnumToOptions(DeviceType, [DeviceType.Unknown])"
            item-title="text"
            item-value="value"
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
import type { Device, DeviceId } from '@/types/devices';
import type { BoardId } from '@/types/hardwares';
import { storeToRefs } from 'pinia';
import { computed, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { VForm } from 'vuetify/components';
import { DeviceType, useDeviceEditComponent } from '@/composables/deviceComposables';
import { Rule } from '@/composables/formComposables';
import { logError, mapEnumToOptions, useRedirect } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useDeviceStore } from '@/stores/deviceStore';
import { useExpanderStore } from '@/stores/expanderStore';

const route = useRoute();
const { redirect } = useRedirect();
const hid = route.query['board'] ? (Number(route.query['board']) as BoardId) : null;
const isEdit = route.name === 'device.edit';

/** Retrieve the device from the URL parameter */
const deviceStore = useDeviceStore();
const id = Number(route.params.id) as DeviceId;
const deviceFromStore = computed<Device>(() =>
  isEdit ? { ...deviceStore.get(id) } : deviceStore.default(hid as BoardId),
);
const device = ref<Device>(deviceFromStore.value);
watch(deviceFromStore, (deviceFromStore) => {
  device.value = { ...deviceFromStore };
});

// Build the hardware selection list:
const { boards } = storeToRefs(useBoardStore());
const { expanders } = storeToRefs(useExpanderStore());
const hardwareItems = computed(() => [
  { group: 'entities.board' },
  ...Object.values(boards.value).map((board) => {
    return { ...board, value: { type: 'Board', id: board.id } };
  }),
  { group: 'entities.expander' },
  ...Object.values(expanders.value).map((expander) => {
    return { ...expander, value: { type: 'Expander', id: expander.id } };
  }),
]);
// const selectHardware = (group: string, id: number) => {
//   device.value.hid = { type: group, id: id } as HardwareId;
// };

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

<style lang="scss" scoped>
.group-item {
  background-color: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
  text-align: center !important;
  text-transform: uppercase !important;
}
</style>
