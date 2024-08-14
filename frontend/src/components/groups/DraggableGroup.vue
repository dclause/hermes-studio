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
  >
    <template #item="{ element }">
      <v-card
        class="wrapper my-2 py-1 px-3"
        :disabled="element.disabled"
        :class="{ disabled: element.disabled }"
      >
        <div class="handler">
          <v-icon icon="mdi-cursor-move" />
        </div>
        <div class="d-flex flex-1-1-100 align-center">
          <actuator v-if="element.device" v-model="actuators[element.device]" />
          <div v-else class="d-flex flex-1-1-100 align-center mt-2 mb-2">
            <v-icon class="ml-2 mr-3" icon="mdi-progress-question" size="30" />
            <div class="group-label font-weight-bold">
              {{ element.name }}
            </div>
          </div>
          <v-btn
            icon="mdi-pencil"
            size="small"
            :to="{
              name: 'device.edit',
              params: { id: element.id },
            }"
            variant="text"
          />
          <v-btn icon="mdi-trash-can" size="small" variant="text" />
        </div>
        <draggable-group v-if="!element.device" v-model="element.children" :on-change="onChange" />
      </v-card>
    </template>
  </draggable>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { MoveEvent } from 'sortablejs';
import { ref } from 'vue';
import draggable from 'vuedraggable';
import { useDeviceStore } from '@/stores/actuatorStore';
import { NestedGroup } from '@/types/groups';

const groups = defineModel<NestedGroup[]>({ required: true });
const props = defineProps<{
  onChange: () => void;
}>();

const { actuators } = storeToRefs(useDeviceStore());

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
  props.onChange();
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
}

.handler {
  grid-column: 1;
  grid-row: 1 / 5;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: move;
}
</style>

<style lang="scss">
.is-dragging .v-drag-area {
  min-height: 1em;
  border: 1px dashed;
  margin: 0 10px 10px 10px;
}
</style>
