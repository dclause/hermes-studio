<template>
  <v-card class="mx-auto pa-4" variant="elevated" max-width="400" width="100%">
    <v-form ref="form" :disabled="loading || !board" :loading="loading" @submit.prevent="onSubmit">
      <!-- Board name -->
      <v-text-field v-model="board.name" label="Name" required :rules="[Rule.REQUIRED]" />

      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <!-- Board type -->
          <v-select
            v-model="model"
            :items="mapEnumToOptions(BoardType, [BoardType.Unknown])"
            item-title="text"
            item-value="value"
            label="Board type"
            required
            :rules="[Rule.REQUIRED, (value: BoardType) => value != BoardType.Unknown]"
            hide-details
            @update:model-value="onModelChange"
          />
        </v-col>
        <v-col class="align-self-center" cols="12" sm="6">
          <component :is="editBoardTypeComponent" v-model="board" />
        </v-col>
      </v-row>

      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <!-- Protocol type -->
          <v-select
            v-model="board.protocol.type"
            :items="mapEnumToOptions(ProtocolType, [ProtocolType.UnknownProtocol])"
            item-title="text"
            item-value="value"
            label="Protocol type"
            required
            :rules="[Rule.REQUIRED, (value: ProtocolType) => value != ProtocolType.UnknownProtocol]"
            hide-details
          />
        </v-col>
        <v-col class="align-self-center" cols="12" sm="6">
          <component :is="editProtocolComponent" v-model="board.protocol" />
        </v-col>
      </v-row>

      <!-- Submit -->
      <v-row>
        <v-col class="align-self-center" cols="12" sm="6">
          <v-btn
            block
            class="mt-2"
            color="primary"
            :disabled="board.model === undefined || board.protocol === undefined || loading"
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
import { computed, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { VForm } from 'vuetify/components';
import {
  BoardType,
  ProtocolType,
  useBoardModelEditComponent,
  useBoardModelType,
  useProtocolEditComponent,
} from '@/composables/boardComposables';
import { Rule } from '@/composables/formComposables';
import { logError, mapEnumToOptions, useRedirect } from '@/composables/globalComposables';
import { useBoardStore } from '@/stores/boardStore';
import { Board, BoardId } from '@/types/boards';

const { redirect } = useRedirect();
const route = useRoute();
const isEdit = route.name === 'board.edit';

const boardStore = useBoardStore();

// Get or create a board from route parameters.
const bid = Number(route.params.bid) as BoardId;
const boardFromStore = computed<Board>(() => {
  if (isEdit) {
    const board = boardStore.get(bid);
    return board ? { ...board } : boardStore.default();
  }
  return boardStore.default();
});

// Define the board to create/edit.
const board = ref<Board>(boardFromStore.value);
const model = ref<BoardType>(useBoardModelType(boardFromStore.value?.model));
watch(boardFromStore, (boardFromStore) => {
  board.value = { ...boardFromStore };
  model.value = useBoardModelType(boardFromStore.model);
});

// Create new form.
const form = ref<VForm>();

// Update the create/edit specific board type component.
const editBoardTypeComponent = computed(() => useBoardModelEditComponent(model.value));
// Update the create/edit specific board protocol component.
const editProtocolComponent = computed(() => useProtocolEditComponent(board.value.protocol.type));

// Validate and submit form
const loading = ref<boolean>(false);
const onSubmit = async () => {
  const { valid } = await form.value!.validate();
  if (valid) {
    loading.value = true;
    isEdit
      ? boardStore
          .update(board.value)
          .then(() => redirect())
          .catch(logError)
      : boardStore
          .create(board.value)
          .then(() => redirect())
          .catch(logError);
    loading.value = false;
  }
};

// Cancel: return to previous page
const onCancel = () => {
  return redirect();
};

// Change the board model on select change.
const onModelChange = (model: string) => {
  if (model) {
    board.value.model = { [model]: '' };
  }
};
</script>

<i18n>
{
  "en": {
    "success": "Successfully created board '{name}' [{id}]"
  },
  "fr": {
    "success": "La carte '{name}' [{id}] a été créée avec succès"
  }
}
</i18n>
