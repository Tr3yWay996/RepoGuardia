<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const saves = ref([]);
const inputText = ref("");
const status = ref("");

onMounted(async () => {
    saves.value = await invoke("list_saves");
});

async function submitBackup() {
    const index = parseInt(inputText.value) - 1;
    if (index >= 0 && index < saves.value.length) {
        status.value = "Backing up...";
        // Assuming your rust command needs the folder name or path
        const selectedSave = saves.value[index][0];
        try {
            await invoke("do_backup", { saveName: selectedSave });
            status.value = `Successfully backed up: ${selectedSave}`;
        } catch (e) {
            status.value = `Error: ${e}`;
        }
    } else {
        status.value = "Invalid number!";
    }
}
</script>

<template>
    <h1>Saves available:</h1>
    <ul>
        <li v-for="([path, date], index) in saves" :key="path">
            {{ index + 1 }}: {{ path }}
            <span class="date">last modified: {{ date }}</span>
        </li>
    </ul>
    <div class="choice_input">
        Enter the number of the save to backup:
        <input type="number" v-model="inputText" placeholder="e.g 1" />
        <button @click="submitBackup">Backup Now</button>
    </div>
    <p v-if="status">{{ status }}</p>
</template>

<style>
.date {
    color: #666;
    font-size: 0.9em;
}
.choice_input {
    margin-top: 20px;
}
input {
    width: 50px;
    margin-right: 10px;
}
</style>
