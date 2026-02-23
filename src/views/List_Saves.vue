<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const saves = ref([])

onMounted(async () => {
  saves.value = await invoke('list_saves')  // [("/path/to/save", "03-02-2026 11:42:15"), ...]
})
</script>

<template>
  <h1>Saves available:</h1>
  <ul>
    <li v-for="([path, date], index) in saves" :key="path">
      {{ index + 1 }}: {{ path }} <span class="date">last modified: {{ date }}</span>
    </li>
  </ul>
</template>

<style scoped>
.date { color: #b1ff5f; font-size: 0.9em; }
</style>
