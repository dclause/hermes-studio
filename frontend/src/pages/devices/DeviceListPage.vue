<template>
  <div class="d-flex align-center mb-4">
    <h1 class="text-h5 text-md-h4 flex-grow-1">
      <v-icon icon="mdi-cog-transfer" />
      {{ t('devices') }}
    </h1>
    <v-btn color="primary" class="mr-5" @click="onCreateGroup">
      <v-icon>mdi-plus</v-icon>
      <span class="d-none d-md-block ml-2">{{ t('new_group') }}</span>
    </v-btn>
    <v-btn color="primary" variant="tonal" class="d-none d-md-flex" :to="{ name: 'device.new' }">
      <v-icon>mdi-plus</v-icon>
      <span class="d-none d-md-block ml-2">{{ t('new_device') }}</span>
    </v-btn>
  </div>

  <v-tabs v-model="tab">
    <v-tab value="boards" @click="router.push({ name: 'board.list' })">
      {{ t('boards') }}
    </v-tab>
    <v-tab value="devices" @click="router.push({ name: 'device.list' })">
      {{ t('devices') }}
    </v-tab>
  </v-tabs>

  <v-spacer class="my-3" />

  <nested-draggable-group
    v-model="draggables"
    :disabled="loading"
    @change="onChange"
    @edit="onEditRequest"
    @delete="onDeleteRequest"
  />

  <confirm-delete-dialog v-model="toBeDeleted" @confirm="onConfirmDelete" />
  <edit-group-dialog v-model="toBeEdited" @confirm="onConfirmCreateOrEdit" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useFlatToNested, useNestedToFlat } from '@/composables/groupComposables';
import router from '@/plugins/router';
import { useDeviceStore } from '@/stores/deviceStore';
import { useGroupStore } from '@/stores/groupStore';
import { Device } from '@/types/devices';
import { FlatGroup, NestedGroup } from '@/types/groups';

const { t } = useI18n();
const groupStore = useGroupStore();
const deviceStore = useDeviceStore();

// Selected tab.
const tab = ref('devices');

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
    // Trick to detect type of toBeDeleted.
    if ('bid' in toBeDeleted.value) {
      deviceStore.delete(toBeDeleted.value.id);
    } else {
      groupStore.delete(toBeDeleted.value.id);
    }
  }
};

// Create / edit a group.
const toBeEdited = ref<FlatGroup | null>(null);
const onEditRequest = (item: NestedGroup) => {
  toBeEdited.value = { ...item };
};
const onCreateGroup = () => (toBeEdited.value = groupStore.default());
const onConfirmCreateOrEdit = () => {
  if (toBeEdited.value) {
    toBeEdited.value.id
      ? groupStore.update(toBeEdited.value.id, toBeEdited.value.name)
      : groupStore.create(toBeEdited.value.name);
  }
};
</script>

<i18n>
{
  "en": {
    "boards": "Boards",
    "devices": "Groups & Controls",
    "new_group": "New group",
    "new_device": "New device",
    "empty": "No group or device configured yet."
  },
  "fr": {
    "boards": "Cartes",
    "devices": "Groups et contrôles",
    "new_group": "Nouveau groupe",
    "new_device": "Nouveau device",
    "empty": "Aucune groupe ou device configurée pour le moment."
  }
}
</i18n>
