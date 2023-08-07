<script setup lang="ts">
import { invoke, window } from '@tauri-apps/api';
import { ref } from 'vue';

var category_on_disk = ref("");

function update_current_selection() {
  invoke("resolve_user_preference_as_string").then((result) => {
    category_on_disk.value = result as string;
  });
}

update_current_selection();

let categories = [
    "neko",
    "bored",
    "cry",
    "facepalm",
    "happy",
    "dance",
    "laugh",
    "smile",
    "blush",
    "handhold",
    "shoot",
    "smug",
    "think",
    "cuddle"
];

let category_image_path = [
    "../src/assets/media/neko.png",
    "../src/assets/media/bored.png",
    "../src/assets/media/cry.png",
    "../src/assets/media/facepalm.png",
    "../src/assets/media/happy.png",
    "../src/assets/media/dance.png",
    "../src/assets/media/laugh.png",
    "../src/assets/media/smile.png",
    "../src/assets/media/blush.png",
    "../src/assets/media/handhold.png",
    "../src/assets/media/shoot.png",
    "../src/assets/media/smug.png",
    "../src/assets/media/think.png",
    "../src/assets/media/cuddle.png",
];

var selected_category = ref(0);

function increment_selection() {
  if (selected_category.value == categories.length - 1) {
    selected_category.value = 0;
  } else {
    selected_category.value++;
  }
}

function apply_preference() {
  var new_pref = categories[selected_category.value];
  invoke("set_preference_from_string", {userPreference: new_pref});
  setTimeout(update_current_selection, 1000);
}

function hide_window() {
  window.appWindow.hide();
}

</script>

<template>
  <div class="container">
    <div class="user-preference-section">
      <div class="section-content">
        <div class="selection-content-child">
          <p id="selection-text">{{ categories[selected_category].toUpperCase() }}</p>
          <button v-on:click="increment_selection()">Next</button>
        </div>
        <img id="category-background-image" :src="category_image_path[selected_category]"/>
      </div>
    </div>
    <div class="action-button-section">
      <div class="row">
        <p class="categories-title">Current selection:</p>
        <p class="categories-title" id="category-on-disk-text">{{ category_on_disk.toUpperCase() }}</p>
      </div>
      <div class="right-buttons">
        <button id="close-button" v-on:click="hide_window()">Close</button>
        <button id="apply-button" v-on:click="apply_preference()">Apply</button>
      </div>
    </div>
  </div>
</template>

<style scoped>

.container {
  height: 100vh;
}

#category-background-image {
  position: absolute;
  z-index: -1;
  width: 100vw;
  height: 100vh;
  transform: translateY(-30px);
  object-fit: cover;
  filter: blur(2px);
}

@media (prefers-color-scheme: dark) {
  #category-background-image {
    filter: blur(2px) brightness(0.7);
  }
}

.user-preference-section {
  height: 80%;
}

.categories-title {
  font-size: 16px;
  margin-top: 6px;
  margin-left: 12px;
}

#category-on-disk-text {
  margin-left: 10px;
  font-weight: 600;
  padding: 5px 12px;
  border-radius: 5px;
  background-color: var(--border-color);
  transform: translateY(-5px);
}

.section-content {
  text-align: center;
  display: flex;
  flex-direction: column;
}

.selection-content-child {
  margin-top: 10%;
}

#selection-text {
  font-size: 40px;
  font-weight: bolder;
  color: white;
  -webkit-text-stroke: 2px #333333;
}

.action-button-section {
  display: flex;
  border-top: 1px solid var(--border-color);
  padding-top: 8px;
  background-color: var(--background-color);
  padding-bottom: 15px;
}

.action-button-section button {
  margin-right: 10px;
}

.right-buttons {
  margin-left: auto;
}

</style>
