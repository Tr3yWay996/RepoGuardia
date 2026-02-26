<template>
    <main class="container">
        <div>
            <h1 class="tittle">Welcome to RepoGuardia.</h1>
            <p class="descrition"
                >A tool to backup your R.E.P.O game saves with everything you
                could need to manage them!</p
            >
        </div>
        <div class="conteuneur-bouton">
            <button style="color:forestgreen"   @click.self="$router.push('/do_backup')">backup save</button>
            <button style="color:goldenrod"        @click.self="$router.push('/manage_backups')">Manage backups</button>
        </div>
    </main>
</template>


<style>
@font-face {
    font-family: 'IBM Plex Serif';
    src: url('../assets/fonts/IMB Plex Serif/IBMPlexSerif-Regular.ttf') format('truetype');
    font-weight: normal;
    font-style: normal;
}
:root {
    font-family: 'IBM Plex Serif', Inter, Avenir, Helvetica, Arial, sans-serif;;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #fffefe;
    background: linear-gradient(-25deg, #4b6cb7 0%, #182848 100%);
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}
.tittle {
    font-size: 3em;
    margin: 0;
}
.descrition {
    font-size: 1em;
    margin: 25px 0 0;
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
    background: rgba(0, 0, 0, 0.25);
    }
</style>