<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";

interface AppConfig {
    default_saves_path: string;
    destination_base_path: string;
    game_version: string;
}

const config = ref<AppConfig>({
    default_saves_path: "",
    destination_base_path: "",
    game_version: "",
});

// Version Modal
const showVersionModal = ref(false);
const versionInput = ref("");

// Saves Path Modal
const showSavesModal = ref(false);
const savesInput = ref("");

// Backup Location Modal
const showBackupModal = ref(false);
const backupInput = ref("");

const loadConfigFromFile = async () => {
    try {
        const loadedConfig = await invoke<any>("load_config");
        config.value = {
            default_saves_path: loadedConfig.default_saves_path || "",
            destination_base_path: loadedConfig.destination_base_path || "",
            game_version: loadedConfig.game_version || "",
        };
    } catch (error) {
        console.error("Failed to load config from file:", error);
    }
};

onMounted(() => {
    loadConfigFromFile();
});

const write_json = async (configObject: AppConfig) => {
    await invoke("save_config", { config: configObject });
};

// Version Modal Functions
const openVersionModal = () => {
    versionInput.value = config.value.game_version;
    showVersionModal.value = true;
};

const saveVersion = async () => {
    config.value.game_version = versionInput.value;
    await write_json(config.value);
    showVersionModal.value = false;
};

const closeVersionModal = () => {
    showVersionModal.value = false;
};

// Saves Path Modal Functions
const openSavesModal = () => {
    savesInput.value = config.value.default_saves_path;
    showSavesModal.value = true;
};

const saveSavesPath = async () => {
    config.value.default_saves_path = savesInput.value;
    await write_json(config.value);
    showSavesModal.value = false;
};

const closeSavesModal = () => {
    showSavesModal.value = false;
};

// Backup Location Modal Functions
const openBackupModal = () => {
    backupInput.value = config.value.destination_base_path;
    showBackupModal.value = true;
};

const saveBackup = async () => {
    config.value.destination_base_path = backupInput.value;
    await write_json(config.value);
    showBackupModal.value = false;
};

const closeBackupModal = () => {
    showBackupModal.value = false;
};


</script>
<template>
    <div class="settings-menu">
        <h1>RepoGuardia settings page</h1>
        <div class="conteuneur-bouton">
            <button style="color: coral" @click="openSavesModal">
                Set the game saves path
            </button>
            <button style="color: coral" @click="openVersionModal">
                Set the game version
            </button>
            <button style="color: coral" @click="openBackupModal">
                Set the backup location
            </button>
        </div>
    </div>

    <!-- Version Modal -->
    <div v-if="showVersionModal" class="modal-overlay">
        <div class="modal-content" @click.stop>
            <h2>Set Game Version</h2>
            <input
                v-model="versionInput"
                type="text"
                placeholder="Enter game version"
                @keyup.enter="saveVersion"
            />
            <div class="modal-buttons">
                <button @click="saveVersion" class="save-btn">Save</button>
                <button @click="closeVersionModal" class="cancel-btn">
                    Cancel
                </button>
            </div>
        </div>
    </div>

    <!-- Saves Path Modal -->
    <div v-if="showSavesModal" class="modal-overlay">
        <div class="modal-content" @click.stop>
            <h2>Set Saves Path</h2>
            <input
                v-model="savesInput"
                type="text"
                placeholder="Enter saves path"
                @keyup.enter="saveSavesPath"
            />
            <div class="modal-buttons">
                <button @click="saveSavesPath" class="save-btn">Save</button>
                <button @click="closeSavesModal" class="cancel-btn">
                    Cancel
                </button>
            </div>
        </div>
    </div>

    <!-- Backup Location Modal -->
    <div v-if="showBackupModal" class="modal-overlay">
        <div class="modal-content" @click.stop>
            <h2>Set Backup Location</h2>
            <input
                v-model="backupInput"
                type="text"
                placeholder="Enter backup location"
                @keyup.enter="saveBackup"
            />
            <div class="modal-buttons">
                <button @click="saveBackup" class="save-btn">Save</button>
                <button @click="closeBackupModal" class="cancel-btn">
                    Cancel
                </button>
            </div>
        </div>
    </div>
</template>

<style scoped>
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
</style>
