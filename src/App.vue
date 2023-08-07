<script setup lang="ts">
import AppConfigurationPane from './views/AppConfigurationView.vue';

document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<template>
  <div class="container">
    <Suspense>
      <AppConfigurationPane></AppConfigurationPane>
    </Suspense>
  </div>
</template>

<style scoped>
</style>
