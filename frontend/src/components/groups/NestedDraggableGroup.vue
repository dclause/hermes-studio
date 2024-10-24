<template>
  <draggable
    v-model="groups"
    :class="{ 'is-dragging': isDragging }"
    class="v-drag-area"
    handle=".mdi-cursor-move"
    tag="div"
    :group="{ name: 'g1' }"
    item-key="id"
    animation="200"
    ghost-class="ghost"
    :move="canMove"
    @move="onMove"
    @change="onChange"
    @delete="onDelete"
    @edit="onEdit"
  >
    <template #item="{ element }">
      <span v-if="element.device" class="ma-3">
        <component
          :is="useDeviceComponent(devices[element.device].type)"
          v-if="devices[element.device]"
          v-model="(devices[element.device] as Actuator).state"
          :device="devices[element.device] as Actuator"
          :disabled="element.disabled"
          :class="{ disabled: element.disabled }"
          class="my-2 pl-3"
          :rounded="0"
          :mode="mode"
          :variant="variant"
          @delete="onDelete"
        >
          <template #prefix>
            <div class="handler handler-device">
              <v-icon icon="mdi-cursor-move" />
            </div>
          </template>
        </component>
      </span>
      <div v-else class="wrapper my-2 pl-3 mr-0" :class="{ disabled: element.disabled }">
        <div class="handler handler-group">
          <v-icon icon="mdi-cursor-move" />
        </div>
        <div>
          <div class="d-flex flex-1-1-100 align-center group-header">
            <div class="d-flex align-center my-2">
              <v-icon class="ml-2 mr-3" icon="mdi-select-group" size="30" />
              <div class="group-label font-weight-bold">
                {{ element.name }}
              </div>
            </div>
            <v-btn icon="mdi-pencil" size="small" variant="text" @click="onEdit(element)" />
            <v-btn icon="mdi-trash-can" size="small" variant="text" @click="onDelete(element)" />
          </div>
          <nested-draggable-group
            v-if="!element.device"
            v-model="element.children"
            :mode="mode"
            :variant="variant"
            @change="onChange"
            @delete="onDelete"
            @edit="onEdit"
          />
        </div>
      </div>
    </template>
  </draggable>
</template>

<script setup lang="ts">
import type { Actuator, Device } from '@/types/devices';
import { storeToRefs } from 'pinia';
import { MoveEvent } from 'sortablejs';
import { ref } from 'vue';
import draggable from 'vuedraggable';
import { useDeviceComponent } from '@/composables/deviceComposables';
import { CommandMode, HardwareMode } from '@/composables/globalComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { NestedGroup } from '@/types/groups';

const deviceStore = useDeviceStore();
const { devices } = storeToRefs(deviceStore);

const emit = defineEmits<{
  change: [];
  edit: [item: NestedGroup];
  delete: [item: NestedGroup | Device];
}>();
const groups = defineModel<NestedGroup[]>({ required: true });
withDefaults(
  defineProps<{
    mode?: HardwareMode;
    variant?: CommandMode;
  }>(),
  {
    mode: HardwareMode.REALTIME,
    variant: CommandMode.FULL,
  },
);

// Flag indicating the user is currently dragging (used to display dropzone).
const isDragging = ref<boolean>(false);

/** Do not allow 'disabled' element to be moved around. */
const canMove = (event: MoveEvent): boolean => {
  return event.related.className.indexOf('disabled') === -1;
};
const onMove = () => {
  isDragging.value = true;
};
const onChange = () => {
  isDragging.value = false;
  emit('change');
};
const onDelete = (item: NestedGroup | Device) => {
  emit('delete', item);
};
const onEdit = (item: NestedGroup) => {
  emit('edit', item);
};
</script>

<style lang="scss" scoped>
.group-label {
  text-overflow: ellipsis;
  display: block;
}

.group-header {
  background: rgb(var(--v-theme-primary-lighten));
  border-top: 2px rgb(var(--v-theme-primary)) solid;
}

.ghost {
  opacity: 0.5;
  background: #c8ebfb;
}

.wrapper {
  display: grid;
  grid-template-columns: 40px auto;
  overflow: auto;
  @media (min-width: 460px) {
    overflow: auto;
  }
}

.handler {
  grid-column: 1;
  grid-row: 1 / 5;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: move;
}

.handler-group {
  border-right: 4px rgb(var(--v-theme-primary)) double;
}
</style>

<style lang="scss">
.v-drag-area {
  min-height: 1em;
  border: 1px transparent dashed;

  .v-drag-area {
    margin: 0 0 10px 10px;
  }
}

.is-dragging .v-drag-area {
  border-color: var(--v-border-color);
}
</style>
