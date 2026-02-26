<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const saves = ref([]);
const status = ref("");
const selectedIndex = ref(-1);

function selectItem(index) {
    selectedIndex.value = index;
}

onMounted(async () => {
    saves.value = await invoke("list_backup");
});

function formatName(raw) {
    const segment = raw.split(/[\/\\]/)[0];
    const withoutPrefix = segment.replace(/^REPO_SAVE_/, "");
    const m = withoutPrefix.match(
        /^(\d{4})_(\d{2})_(\d{2})_(\d{2})_(\d{2})_(\d{2})/,
    );
    if (m) return `${m[1]}-${m[2]}-${m[3]}  ${m[4]}:${m[5]}:${m[6]}`;
    return segment;
}

async function submitBackupDelete() {
    if (selectedIndex.value < 0 || selectedIndex.value >= saves.value.length) {
        status.value = "Select a backup first!";
        return;
    }
    status.value = "Deleting backup...";
    const selectedSave = saves.value[selectedIndex.value][0];
    try {
        await invoke("do_delete", { saveName: selectedSave });
        saves.value.splice(selectedIndex.value, 1);
        selectedIndex.value = -1;
        status.value = "Backup deleted successfully.";
    } catch (e) {
        status.value = `Error: ${e}`;
    }
}
</script>

<template>
    <div class="view-layout">
        <h1>Backup available:</h1>
        <ul>
            <li
                v-for="([path, date, label], index) in saves"
                :key="path"
                @click="selectItem(index)"
                :class="{ selected: selectedIndex === index }"
            >
                {{ index + 1 }}: {{ formatName(path) }}
                <span v-if="label" class="label"> [{{ label }}]   </span>
                <span class="date">last modified: {{ date }}</span>
            </li>
        </ul>
        <div class="choice_input">
            <button @click="submitBackupDelete" :disabled="selectedIndex === -1">Delete Now</button>
        </div>
        <p v-if="status">{{ status }}</p>
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
    list-style: none;
}
li {
    cursor: pointer;
    padding: 4px 6px;
    border-radius: 4px;
}
li:hover {
    background: rgba(255, 255, 255, 0.1);
}
li.selected {
    background: rgba(255, 80, 80, 0.2);
    outline: 1px solid red;
}
.date {
    color: red;
    font-size: 0.9em;
}
.label {
    color: #ffa3ef;
    font-size: 0.9em;
    font-style: italic;
}
.choice_input {
    flex-shrink: 0;
    margin-top: 10px;
    padding-bottom: 10px;
}
input {
    width: 50px;
    margin-right: 10px;
}
</style>
