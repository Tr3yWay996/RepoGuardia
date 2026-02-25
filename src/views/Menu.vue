<script setup lang="ts">
/* import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
const testMsg = ref("");
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
async function delete_backups() {
  
}
async function mystery() {
    console.log('fak u')
} */
</script>

<template>
    <main class="container">
        <div>
            <h1>Welcome to RepoGuardia.</h1>
            <p
                >A tool to backup your R.E.P.O game saves with everything you
                could need to manage them!</p
            >
        </div>
        <div class="conteuneur-bouton">
            <button style="color:bisque"        @click.self="$router.push('/saves_list')">List all saves</button>
            <button style="color:orange"        @click.self="$router.push('/do_restore')">Restore backup</button>
            <button style="color:bisque"        @click.self="$router.push('/backup_list')">List all backups</button>
            <button style="color:red"           @click.self="$router.push('/do_delete')">Delete backup</button>
            <button style="color:greenyellow"   @click.self="$router.push('/do_backup')">backup save</button>
<!--             <button style="color:purple"        @click.self="mystery">Mystery button</button>
            <button style="color:brown"         @click.self="open_konsole">Shell execution :3</button> -->
        </div>
<!--         <p>{{ testMsg }}</p>
        <p>{{ resetCountMsg }}</p> -->
    </main>
</template>


<style>
:root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #fffefe;
    background: #043d00;
    background: linear-gradient(25deg, rgb(5, 72, 0) 0%, rgba(54, 0, 0, 1) 50%, rgba(2, 0, 99, 1) 100%);
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