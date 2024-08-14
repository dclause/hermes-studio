<template>
  <draggable-group v-model="draggables" :disabled="loading" :on-change="onChange" />
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { ref, watch } from 'vue';
import { useFlatToNested, useNestedToFlat } from '@/composables/groupComposables';
import { useGroupStore } from '@/stores/groupStore';
import { NestedGroup } from '@/types/groups';

const groupStore = useGroupStore();
const { groups, loading } = storeToRefs(groupStore);
const draggables = ref<NestedGroup[]>(useFlatToNested(groups.value));

// When groups are updated (page load, or grouped saved, ...): rebuild draggables.
watch(groups, (groups) => {
  draggables.value = useFlatToNested(groups);
});

// Save the draggables when moved.
const onChange = () => {
  groupStore.save(useNestedToFlat(draggables.value));
};
</script>
