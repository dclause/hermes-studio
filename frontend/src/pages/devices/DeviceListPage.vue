<template>
  <draggable-group v-model="draggables" :disabled="loading" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useFlatToNested, useNestedToFlat } from '@/composables/groupComposables';
import { useGroupStore } from '@/stores/groupStore';
import { NestedGroup } from '@/types/groups';

const groupStore = useGroupStore();
const { groups, loading } = storeToRefs(groupStore);
const draggables = ref<NestedGroup[]>([]);

// When groups is loaded (only used in case of page refresh, hence the "once" watcher).
watch(
  groups,
  (groups) => {
    draggables.value = useFlatToNested(groups);
  },
  { once: true },
);
// When draggables updates: save to groups.
watch(draggables, (draggables) => {
  groupStore.save(useNestedToFlat(draggables));
});
</script>
