<template>
  <v-card class="mx-auto pa-4" variant="elevated" max-width="600" width="100%">
    <v-form
      ref="form"
      :disabled="loading || !expander"
      :loading="loading"
      @submit.prevent="onSubmit"
    >
      <v-text-field v-model="expander.name" label="Name" required :rules="[Rule.REQUIRED]" />

      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-select
            v-model="expander.hid.id"
            :items="boardItems"
            item-title="name"
            item-value="id"
            label="Board"
            required
            :rules="[Rule.REQUIRED]"
            :disabled="isEdit"
          />
        </v-col>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-select
            v-model="expander.type"
            :items="mapEnumToOptions(ExpanderType, [ExpanderType.Unknown])"
            item-title="text"
            item-value="value"
            label="Expander type"
            required
            :rules="[Rule.REQUIRED, (value: ExpanderType) => value != ExpanderType.Unknown]"
            :disabled="isEdit"
          />
        </v-col>
      </v-row>
      <component :is="editComponent" v-model="expander" />

      <!-- Submit -->
      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-btn
            block
            class="mt-2"
            color="primary"
            :disabled="!expander.type || loading"
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
import type { BoardId, Expander, ExpanderId } from '@/types/hardwares';
import { storeToRefs } from 'pinia';
import { computed, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { VForm } from 'vuetify/components';
import { ExpanderType, useExpanderEditComponent } from '@/composables/expanderComposables';
import { Rule } from '@/composables/formComposables';
import { logError, mapEnumToOptions, useRedirect } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { useExpanderStore } from '@/stores/expanderStore';

const route = useRoute();
const { redirect } = useRedirect();
const hid = route.query['board'] ? (Number(route.query['board']) as BoardId) : null;
const isEdit = route.name === 'expander.edit';

/** Retrieve the expander from the URL parameter */
const expanderStore = useExpanderStore();
const id = Number(route.params.id) as ExpanderId;
const expanderFromStore = computed<Expander>(() =>
  isEdit ? { ...expanderStore.get(id) } : expanderStore.default(hid as BoardId),
);
const expander = ref<Expander>(expanderFromStore.value);
watch(expanderFromStore, (expanderFromStore) => {
  expander.value = { ...expanderFromStore };
});

// Build the board selection.
const { boards } = storeToRefs(useBoardStore());
const boardItems = computed(() => Object.values(boards.value));

// Create new form.
const form = ref<VForm>();

// Update the create/edit specific expander type component.
const editComponent = computed(() => useExpanderEditComponent(expander.value.type));

// Save the newly created expander.
const loading = ref<boolean>(false);
const onSubmit = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    loading.value = true;
    isEdit
      ? expanderStore
          .update(expander.value)
          .then(() => redirect())
          .catch(logError)
      : expanderStore
          .create(expander.value)
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
