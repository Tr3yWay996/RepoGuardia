<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const saves = ref([])
const status = ref('')

onMounted(async () => {
  saves.value = await invoke('list_backup')
})

async function restore(path) {
  status.value = `Restoring ${path}...`
  try {
    await invoke('do_restore', { saveName: path })
    status.value = `Restored ${path} successfully!`
  } catch (e) {
    status.value = `Error: ${e}`
  }
}
</script>

<template>
  <h1>Backups available:</h1>
  <ul>
    <li v-for="([path, date], index) in saves" :key="path">
      {{ index + 1 }}: {{ path }} <span class="date">last modified: {{ date }}</span>
      <button @click="restore(path)">Restore</button>
    </li>
  </ul>
  <p v-if="status" class="status">{{ status }}</p>
</template>

<style scoped>
.date { color: #fff268; font-size: 0.9em; }
.status { color: #aaffaa; margin-top: 1em; }
</style>
