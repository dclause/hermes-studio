<template>
  <div v-for="group in groups" :key="group.id">
    <div v-if="group.device">
      <component
        :is="useDeviceComponent(devices[group.device].type)"
        v-if="devices[group.device]"
        v-model="(devices[group.device] as Actuator).state"
        :device="devices[group.device] as Actuator"
        @delete="onDelete"
      />
    </div>
    <div
      v-else-if="shouldDisplay(group)"
      :class="{ 'ml-5': group.level }"
      class="group pt-1 pb-2 pl-3 my-2"
    >
      <div class="d-flex flex-1-1-100 align-center mb-2">
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
import type { Actuator, Device } from '@/types/devices';
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

const shouldDisplay = (group: NestedGroup) => {
  // A group that have device should be displayed.
  if (group.device) {
    return true;
  }
  let display = false;
  group.children.forEach((child) => {
    display = display || shouldDisplay(child);
  });
  return display;
};
</script>

<style lang="scss" scoped>
.group {
  border-left: 4px double rgb(var(--v-theme-primary));
}

.group-label {
  width: 10rem;
  text-overflow: ellipsis;
  display: block;
}
</style>
