<script setup lang="ts">
import { ref, VueElement } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const testMsg = ref("");
const count = ref(0);
const resetCountMsg = ref("");
const trigger = ref(false);
async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value });
}
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
            <button @click.self="test">test</button>
            <button @click.self="resetCount">reset count</button>
            <button @click.self="open_konsole">open konsole</button>
            <button @click.self="open_konsole">open konsole</button>
        </div>
        <p>{{ testMsg }}</p>
        <p>{{ resetCountMsg }}</p>
    </main>
</template>
<style scoped>
.conteuneur-bouton {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
    justify-content: center;
    font-size: x-large;
}
.conteuneur-bouton > button {
    flex: 0 0 calc(50% - 5px); /* 2 per row */
}
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

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

.logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: 0.75s;
}

.logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
}

.row {
    display: flex;
    justify-content: center;
}

a {
    font-weight: 500;
    color: #646cff;
    text-decoration: inherit;
}

a:hover {
    color: #535bf2;
}

h1 {
    text-align: center;
}

input,
button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
    cursor: pointer;
}

button:hover {
    border-color: #396cd8;
}
button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
}

input,
button {
    outline: none;
}

#greet-input {
    margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
    :root {
        color: #f6f6f6;
        background-color: #2f2f2f;
    }

    a:hover {
        color: #24c8db;
    }

    input,
    button {
        color: #ffffff;
        background-color: #0f0f0f98;
    }
    button:active {
        background-color: #0f0f0f69;
    }
}
</style>
