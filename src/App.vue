<script setup lang="ts">
import AppConfigurationPane from './views/AppConfigurationView.vue';
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
