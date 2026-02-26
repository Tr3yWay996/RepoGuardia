<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const saves = ref([]);
const status = ref("");
const showLabelModal = ref(false);
const labelInput = ref("");
const selectedIndex = ref(-1);

function handleKeydown(e) {
    if (showLabelModal.value) {
        if (e.key === "Escape") closeLabelModal()
        return
    }
    if (e.key === "Delete") {
        submitBackupDelete()
    }
    if (e.key === "Enter") {
        submitRestore()
    }
    if (e.key === " ") {
        openLabelModal()
    }
}

function selectItem(index) {
    selectedIndex.value = index;
}

onMounted(async () => {
    saves.value = await invoke("list_backup");
    document.addEventListener("keydown", handleKeydown)
});
onUnmounted(() => {
    document.removeEventListener("keydown", handleKeydown)
})

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
async function submitRestore() {
    if (selectedIndex.value < 0 || selectedIndex.value >= saves.value.length) {
        status.value = "Select a backup first!";
        return;
    }
    status.value = "Restoring backup...";
    const selectedSave = saves.value[selectedIndex.value][0];
    try {
        await invoke("do_restore", { saveName: selectedSave });
        saves.value.splice(selectedIndex.value, 1);
        selectedIndex.value = -1;
        status.value = "Backup restored successfully.";
    } catch (e) {
        status.value = `Error: ${e}`;
    }
}
function openLabelModal() {
    if (selectedIndex.value >= 0 && selectedIndex.value < saves.value.length) {
        labelInput.value = saves.value[selectedIndex.value][2] || "";
        showLabelModal.value = true;
        status.value = "";
    } else {
        status.value = "Select a backup first!";
    }
}

async function saveLabel() {
    const selectedSave = saves.value[selectedIndex.value][0];
    try {
        status.value = "Saving label...";
        await invoke("do_rename", { saveName: selectedSave, newLabel: labelInput.value });
        saves.value[selectedIndex.value][2] = labelInput.value;
        status.value = `Label saved for: ${formatName(selectedSave)}`;
    } catch (e) {
        status.value = `Error: ${e}`;
    }
    showLabelModal.value = false;
}

function closeLabelModal() {
    showLabelModal.value = false;
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
            <button @click="submitRestore" :disabled="selectedIndex === -1">Restore Now</button>
            <button @click="openLabelModal" :disabled="selectedIndex === -1">Set Label</button>

        </div>
        <p v-if="status">{{ status }}</p>
    </div>
    <div v-if="showLabelModal" class="modal-overlay">
        <div class="modal-content" @click.stop>
            <h2>Set Backup Label</h2>
            <input
                v-model="labelInput"
                type="text"
                placeholder="Enter backup label"
                autofocus
                @keyup.enter="saveLabel"
                class="label-input"
            />
            <div class="modal-buttons">
                <button @click="saveLabel" class="save-btn">Save</button>
                <button @click="closeLabelModal" class="cancel-btn">Cancel</button>
            </div>
        </div>
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
    background: rgba(255, 0, 0, 0.499);
    outline:1px solid rgb(0, 76, 255);
}
.date {
    color: rgb(255, 242, 0);
    font-size: 0.9em;
}
.label {
    color: #ffa3ef;
    font-size: 0.9em;
    font-style: italic;
}
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
}
.modal-content {
    background-color: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    max-width: 400px;
    width: 90%;
}
.modal-content h2 {
    margin-top: 0;
    margin-bottom: 15px;
    color: #333;
}
.modal-content input {
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
    box-sizing: border-box;
    margin-bottom: 15px;
}
.modal-content input:focus {
    outline: none;
    border-color: coral;
    box-shadow: 0 0 5px rgba(255, 127, 80, 0.3);
}
.modal-buttons {
    display: flex;
    gap: 10px;
    justify-content: flex-end;
}
.save-btn,
.cancel-btn {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.2s;
}
.save-btn {
    background-color: coral;
    color: white;
}
.save-btn:hover {
    background-color: #ff7f50;
}
.cancel-btn {
    background-color: #ddd;
    color: #333;
}
.cancel-btn:hover {
    background-color: #ccc;
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
