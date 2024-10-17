<template>
  <div class="text-center">
    <h1>Hermes-Studio</h1>

    <v-alert
      variant="tonal"
      class="starting"
      border="start"
      border-color="deep-purple accent-4"
      color="deep-purple-accent-4"
      elevation="2"
    >
      <div class="font-weight-bold pb-2">
        {{ t('intro') }}
      </div>
      <ol>
        <li>
          <v-icon class="text-black">
            mdi-cog-transfer
          </v-icon>
          {{ t('configure') }}
          <a
            class="text-decoration-none font-weight-medium font-italic"
            href="https://dclause.github.io/hermes-studio/userguide/configuration-panel"
            target="_blank"
          >
            ({{ t('learn') }})</a>.
        </li>
        <li>
          <v-icon class="text-black">
            mdi-camera-control
          </v-icon>
          {{ t('control') }}
          <a
            class="text-decoration-none font-weight-medium font-italic"
            href="https://dclause.github.io/hermes-studio/userguide/control-panel"
            target="_blank"
          >({{ t('more') }})</a>.
        </li>
        <li>
          <v-icon class="text-black">
            mdi-movie-open
          </v-icon>
          {{ t('timeline') }}
          <a
            class="text-decoration-none font-weight-medium font-italic"
            href="https://dclause.github.io/hermes-studio/userguide/timeline-panel"
            target="_blank"
          >({{ t('discover') }})</a>.
        </li>
      </ol>
    </v-alert>

    <div>{{ t('license') }}</div>
    <div>Copyright © 2024-present Dominique CLAUSE</div>
  </div>
</template>

<script lang="ts" setup>
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<style lang="scss" scoped>
.starting {
  text-align: left;
  font-size: 1.2em;
  max-width: 700px;
  margin: 2em auto;
  padding-left: 1em;

  ol {
    //color: initial !important;
    margin-left: 3em;

    a {
      color: #551a8b !important;
    }
  }
}
</style>

<i18n>
{
  "en": {
    "intro": "Use the panels from the toolbar to get started:",
    "configure": "Configure your robot",
    "control": "Control your robot remotely in real-time",
    "timeline": "Create and play amazing animations",
    "learn": "learn how",
    "more": "more about it",
    "discover": "discover how",
    "license": "Released under the MIT License."
  },
  "fr": {
    "intro": "Utilisez les panels depuis la barre d'outils pour commencer :",
    "configure": "Configurez votre robot",
    "control": "Contrôlez votre robot à distance en temps réel",
    "timeline": "Créez et jouez des animations incroyables",
    "learn": "apprenez comment",
    "more": "en savoir plus",
    "discover": "découvrez comment",
    "license": "Publié sous la licence MIT."
  }
}
</i18n>
