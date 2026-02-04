<script setup lang="ts">
async function saves_location() {
    console.log("clicked the saves path setting button")
}
async function game_version() {
    console.log("clicked game version setting button")
}

</script>
<template>
    <div class="settings-menu">
        <h1>
            RepoGuardia settings page
        </h1>
        <div class="conteuneur-bouton">
            <button style="color:coral"    @click.self="saves_location">Set the game saves path</button>
            <button style="color:coral"    @click.self="game_version">Set the game version</button>
        </div>
    </div>
</template>