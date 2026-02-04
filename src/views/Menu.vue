<script setup lang="ts">
import { ref, stop, VueElement } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { app } from "@tauri-apps/api";
import { getTauriVersion } from "@tauri-apps/api/app";

const greetMsg = ref("");
const name = ref("");
const testMsg = ref("");
const count = ref(0);
const resetCountMsg = ref("");
const trigger = ref(false);
const version = ("")
async function test() {
    count.value += 1;
    testMsg.value = await invoke("test", {
        test: `Count value: ${count.value}`,
    });
}
async function resetCount() {
    count.value = 0;
    testMsg.value = await invoke("test", {
        test: `Reseted count to ${count.value}`,
    });
}
async function open_konsole() {
    trigger.value = true;
    await invoke("open_konsole");
}
async function list_saves() {
  
}
async function restore_backup() {
  
}
async function list_backups() {
  
}
async function delete_backups() {
  
}
async function backup_saves() {
  
}
async function mystery() {
    console.log('fak u')
}
</script>

<template>
    <main class="container">
        <div>
            <h1>Welcome to RepoGuardia.</h1>
            <p1
                >A tool to backup your R.E.P.O game saves with everything you
                could need to manage them!</p1
            >
        </div>
        <div class="conteuneur-bouton">
            <button style="color:bisque"        @click.self="$router.push('/saves_list')">List all saves</button>
            <button style="color:orange"        @click.self="restore_backup">Restore backup</button>
            <button style="color:bisque"        @click.self="list_backups">List all backups</button>
            <button style="color:red"           @click.self="delete_backups">Delete backup</button>
            <button style="color:greenyellow"   @click.self="backup_saves">backup save</button>
            <button style="color:purple"        @click.self="mystery">Mystery button</button>
            <button style="color:brown"         @click.self="open_konsole">Shell execution :3</button>
        </div>
        <p>{{ testMsg }}</p>
        <p>{{ resetCountMsg }}</p>
    </main>
</template>


<style>
:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #fffefe;
    background-image: url(/Fond-RepoGuardia.png);
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

.container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
}
.conteuneur-bouton {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    justify-content: center;
    font-size: x-large;
}
.conteuneur-bouton > button {
    flex: 0 0 calc(50% - 5px); /* 2 per row */
    background-color: rgba(0, 0, 64, 0);
}
</style>