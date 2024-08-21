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
  >
    <template #item="{ element }">
      <div v-if="element.device">
        <component
          :is="useDeviceComponent(devices[element.device].type)"
          v-if="devices[element.device]"
          v-model="devices[element.device] as Actuator"
          :disabled="element.disabled"
          :class="{ disabled: element.disabled }"
          class="my-2 px-3"
          @delete="onDelete"
        >
          <template #prefix>
            <div class="handler">
              <v-icon icon="mdi-cursor-move" />
            </div>
          </template>
        </component>
      </div>
      <v-card
        v-else
        class="wrapper my-2 px-3"
        :disabled="element.disabled"
        :class="{ disabled: element.disabled }"
      >
        <div class="handler">
          <v-icon icon="mdi-cursor-move" />
        </div>
        <div>
          <div class="d-flex flex-1-1-100 align-center">
            <div class="d-flex flex-1-1-100 align-center my-2">
              <v-icon class="ml-2 mr-3" icon="mdi-select-group" size="30" />
              <div class="group-label font-weight-bold">
                {{ element.name }}
              </div>
            </div>
            <v-btn icon="mdi-pencil" size="small" variant="text" @click="emit('edit', element)" />
            <v-btn
              icon="mdi-trash-can"
              size="small"
              variant="text"
              @click="emit('delete', element)"
            />
          </div>
          <nested-draggable-group
            v-if="!element.device"
            v-model="element.children"
            @change="onChange"
            @delete="onDelete"
          />
        </div>
      </v-card>
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
</script>

<style lang="scss" scoped>
.group-label {
  width: 10rem;
  text-overflow: ellipsis;
  display: block;
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
</style>

<style lang="scss">
.handler {
  grid-column: 1;
  grid-row: 1 / 5;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: move;
}

.v-drag-area {
  min-height: 1em;
  border: 1px transparent dashed;
  margin: 0 10px 10px 10px;
}

.is-dragging .v-drag-area {
  border-color: var(--v-border-color);
}
</style>
