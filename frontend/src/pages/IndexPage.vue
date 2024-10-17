<template>
  <div class="text-center">
    <h1>HermesStudio</h1>

    <v-alert
      variant="tonal"
      class="starting"
      border="start"
      border-color="deep-purple accent-4"
      color="deep-purple-accent-4"
      elevation="2"
    >
      <div class="font-weight-bold pb-2">
        Use the panels from the toolbar to get started:
      </div>
      <ol>
        <li>
          <v-icon class="text-black">
            mdi-cog-transfer
          </v-icon>
          Configure your robot
          <a
            class="text-decoration-none font-weight-medium font-italic"
            href="https://dclause.github.io/hermes-studio/userguide/configuration-panel"
            target="_blank"
          >
            (learn how)</a>.
        </li>
        <li>
          <v-icon class="text-black">
            mdi-camera-control
          </v-icon>
          Control your robot remotely in real-time
          <a
            class="text-decoration-none font-weight-medium font-italic"
            href="https://dclause.github.io/hermes-studio/userguide/control-panel"
            target="_blank"
          >(more about it)</a>.
        </li>
        <li>
          <v-icon class="text-black">
            mdi-movie-open
          </v-icon>
          Create and play amazing animations
          <a
            class="text-decoration-none font-weight-medium font-italic"
            href="https://dclause.github.io/hermes-studio/userguide/timeline-panel"
            target="_blank"
          >(discover how)</a>.
        </li>
      </ol>
    </v-alert>

    <div>Released under the MIT License.</div>
    <div>Copyright © 2024-present Dominique CLAUSE</div>
  </div>
</template>

<script lang="ts" setup></script>

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
  }
}
</style>
