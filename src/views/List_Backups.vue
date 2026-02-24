<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const saves = ref([])
const status = ref('')

onMounted(async () => {
  saves.value = await invoke('list_backup')
})

function formatName(raw) {
  const segment = raw.split(/[\/\\]/)[0]
  const withoutPrefix = segment.replace(/^REPO_SAVE_/, '')
  const m = withoutPrefix.match(/^(\d{4})_(\d{2})_(\d{2})_(\d{2})_(\d{2})_(\d{2})/)
  if (m) return `${m[1]}-${m[2]}-${m[3]}  ${m[4]}:${m[5]}:${m[6]}`
  return segment
}
</script>

<template>
  <div class="view-layout">
    <h1>Backups available:</h1>
    <ul>
      <li v-for="([path, date], index) in saves" :key="path">
        {{ index + 1 }}: {{ formatName(path) }} <span class="date">last modified: {{ date }}</span>
      </li>
    </ul>
    <p v-if="status" class="status">{{ status }}</p>
  </div>
</template>

<style scoped>
.view-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
  box-sizing: border-box;
  padding: 0 1rem;
}
.view-layout h1 {
  flex-shrink: 0;
}
.view-layout ul {
  flex: 1;
  overflow-y: auto;
  min-height: 0;
  margin: 0;
  padding: 0 0 0 1.2rem;
}
.date { color: #fff268; font-size: 0.9em; }
.status { color: #aaffaa; margin-top: 1em; flex-shrink: 0; }
</style>
