<template>
  <div class="d-flex align-center mb-4">
    <h1 class="text-h5 text-md-h4 flex-grow-1">
      <v-icon icon="mdi-camera-control" />
      {{ t('title') }}
    </h1>
    <v-btn class="mr-5" color="primary" :icon="$vuetify.display.xs === true" @click="onCreateGroup">
      <v-icon>mdi-plus</v-icon>
      <span class="d-none d-sm-block">{{ t('new_group') }}</span>
    </v-btn>
    <v-btn :icon="$vuetify.display.xs === true" :to="{ name: 'device.new' }">
      <v-icon>mdi-plus</v-icon>
      <span class="d-none d-sm-block">{{ t('new_device') }}</span>
    </v-btn>
  </div>

  <nested-draggable
    v-model="draggables"
    :disabled="loading"
    @change="onChange"
    @edit="onEditRequest"
    @delete="onDeleteRequest"
  />

  <confirm-delete-dialog v-model="toBeDeleted" @confirm="onConfirmDelete" />
  <create-group-dialog v-model="toBeCreated" @confirm="onConfirmCreate" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useFlatToNested, useNestedToFlat } from '@/composables/groupComposables';
import { useDeviceStore } from '@/stores/deviceStore';
import { useGroupStore } from '@/stores/groupStore';
import { Device } from '@/types/devices';
import { FlatGroup, NestedGroup } from '@/types/groups';

const { t } = useI18n();
const groupStore = useGroupStore();
const deviceStore = useDeviceStore();

const { groups, loading } = storeToRefs(groupStore);
const draggables = ref<NestedGroup[]>(useFlatToNested(groups.value));

// When groups are updated (page load, or grouped saved, ...): rebuild draggables.
watch(
  groups,
  (groups) => {
    draggables.value = useFlatToNested(groups);
  },
  { deep: true },
);

// Save the draggables when moved.
const onChange = () => {
  groupStore.save(useNestedToFlat(draggables.value));
};

// Delete a group / device.
const toBeDeleted = ref<NestedGroup | Device | null>(null);
const onDeleteRequest = (item: NestedGroup | Device) => (toBeDeleted.value = item);
const onConfirmDelete = () => {
  if (toBeDeleted.value) {
    // Trick to detect a Device: it has a 'bid' attribute.
    if ('bid' in toBeDeleted.value) {
      deviceStore.delete(toBeDeleted.value.id);
    } else {
      groupStore.delete(toBeDeleted.value.id);
    }
  }
};

// Create a group.
const toBeCreated = ref<FlatGroup | null>(null);
const onEditRequest = (item: NestedGroup) => (toBeCreated.value = { ...item });
const onCreateGroup = () => (toBeCreated.value = groupStore.default());
const onConfirmCreate = () => {
  if (toBeCreated.value) {
    groupStore.create(toBeCreated.value);
  }
};
</script>

<i18n>
{
  "en": {
    "title": "Groups & Controls",
    "new_group": "New group",
    "new_device": "New device",
    "empty": "No group or device configured yet."
  },
  "fr": {
    "title": "Groups et contrôles",
    "new_group": "Nouveau groupe",
    "new_device": "Nouveau device",
    "empty": "Aucune groupe ou device configurée pour le moment."
  }
}
</i18n>
