<template>
  <div v-for="group in groups" :key="group.id">
    <div v-if="group.device">
      <component
        :is="useDeviceComponent(devices[group.device as DeviceId].type)"
        v-if="devices[group.device as DeviceId]"
        v-model="devices[group.device as DeviceId] as Actuator"
        @delete="onDelete"
      />
    </div>
    <div v-else :class="{ 'ml-5': group.level > 1 }" class="group px-2 my-2">
      <div class="d-flex flex-1-1-100 align-center my-2">
        <v-icon class="mr-3" icon="mdi-select-group" size="30" />
        <div class="group-label font-weight-bold">
          {{ group.name }}
        </div>
      </div>
      <nested-group v-model="group.children" @delete="onDelete" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Actuator, Device, DeviceId } from '@/types/devices';
import type { NestedGroup } from '@/types/groups';
import { storeToRefs } from 'pinia';
import { useDeviceComponent } from '@/composables/deviceComposables';
import { useDeviceStore } from '@/stores/deviceStore';

const deviceStore = useDeviceStore();
const { devices } = storeToRefs(deviceStore);
const groups = defineModel<NestedGroup[]>({ required: true });

const emit = defineEmits<{
  delete: [item: Device];
}>();
const onDelete = (item: Device) => {
  emit('delete', item);
};
</script>

<style lang="scss" scoped>
.group {
  border: 1px dashed;
}

.group-label {
  width: 10rem;
  text-overflow: ellipsis;
  display: block;
}
</style>
