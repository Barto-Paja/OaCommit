<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import "/src/assets/styles.css";

interface Commit {
  hash: string;
  message: string;
  author: string;
  date: number;
  branches: string[];
  parents: string[];
}

const selectedCommit = ref<Commit | null>(null);
const name = ref("");
const commits = ref<Commit[]>([]);
const local_branches = ref([]);
const remote_branches = ref([]);

async function get_git_log() {
  try {
    const r = await open({
      directory: true, // Wybór folderu zamiast pliku
      multiple: false, // Pozwala wybrać tylko jeden folder
    });

    if (r) {
      console.info(r);
      name.value = r;
      commits.value = await invoke("get_git_log", { repoPath: r });
      console.log(name);

      local_branches.value = await invoke("get_local_branches", {repoPath: r});
      remote_branches.value = await invoke("get_remote_branches", {repoPath: r});

    }

  } catch (error) {
    console.error("Błąd:", error);
  }
}

async function git_init() {
  try {
    const r = await open({
      directory: true, // Wybór folderu zamiast pliku
      multiple: false, // Pozwala wybrać tylko jeden folder
    });
    if (r) {
      await invoke("git_init", { repoPath: r });
    }
  } catch (e) {
    console.error("Błąd:", e);
  }
}

function format_date(timestamp: number) : string {
  return new Date(timestamp*1000).toLocaleString();
}

</script>

<template>
  <main class="container">
    <div class="header">
      <img class="header-icon" src="/src/assets/electrodes.gif" alt="Animacja" />
      <div v-if="commits.length > 0">
        <div class="menu">
          <div>{{ name }}</div>
          <form @submit.prevent="get_git_log"><button class="menu-item">Open Ctrl+O</button></form>
          <form @submit.prevent="git_init"><button class="menu-item">Init Ctrl+I</button></form>
          <form @submit.prevent="get_git_log"><button class="menu-item">Clone Ctrl+N</button></form>
        </div>
      </div>
    </div>
    <div class="branches">
      <div class="branches-label-header"><img class="branches-label-icon" src="/src/assets/tree.gif" alt="Animacja" /><div>Local Branches</div></div>
      <div class="branches-label-header" v-for="branch in local_branches">
        <img class="branches-label-icon" src="/src/assets/leaf_12586785.gif" alt="Animacja" /><div>{{ branch[1] }}</div>
      </div>
      <div class="branches-label-header"><img class="branches-label-icon" src="/src/assets/tree_12586813.gif" alt="Animacja" /><div>Remote Branches</div></div>
      <div class="branches-label-header" v-for="branch in remote_branches">
        <img class="branches-label-icon" src="/src/assets/leaf_12586785.gif" alt="Animacja" /><div>{{ branch[1] }}</div>
      </div>
    </div>
    <div class="content">
      <div v-if="commits.length === 0" class="empty-message">
        <div><img class="empty-icon" src="/src/assets/share.gif" alt="Animacja" /><div>Open Repo, Clone Remote or Init New</div></div>
        <div class="menu">
          <div>{{ name }}</div>
          <form @submit.prevent="get_git_log"><button class="menu-item">Open Ctrl+O</button></form>
          <form @submit.prevent="git_init"><button class="menu-item">Init Ctrl+I</button></form>
          <form @submit.prevent="get_git_log"><button class="menu-item">Clone Ctrl+N</button></form>
        </div>
      </div>
      <div class="row" v-for="commit in commits">
        <div class="cell-dot" >
          <div
              @mouseover="selectedCommit = commit"
          >
            <img class="cell-dot" src="/src/assets/share.png" alt="Animacja" />
          </div>
        </div>
        <div class="cell-hash-label">
          <div :class="'commit-hash-label'">
            {{ commit.hash }}
          </div>
        </div>
        <div class="cell-date-label">
          <div>
            {{ format_date(commit.date) }} {{ commit.message }}
          </div>
        </div>
        <div class="cell-branches-label">
          <div v-for="(branch, index) in commit.branches" :class="'branch-label-'+index%3">
            {{branch}}
          </div>
        </div>
      </div>
    </div>
    <div class="commit-details" v-if="selectedCommit">
      <div class="commit-hash">{{ selectedCommit.hash }}</div>
<!--      <div>{{ selectedCommit.branch }}:</div>-->
      <div>{{ selectedCommit.message }}</div>
      <div>Author: {{ selectedCommit.author }}</div>
      <form class="row" @submit.prevent="selectedCommit=null">
        <button type="submit">Greet</button>
      </form>
    </div>
  </main>
</template>



<style>

.cell-dot {
  flex: 1; /* Każda komórka zajmie równą przestrzeń */
  min-height: 48px; /* Wysokość minimalna */
  /*border: 1px solid #ccc; */
  max-width: 48px;
}
.cell-hash-label {
  flex: 1; /* Każda komórka zajmie równą przestrzeń */
  min-height: 24px; /* Wysokość minimalna */
  /*border: 1px solid #ccc; */
  padding: 10px;
}
.cell-date-label {
  flex: 4; /* Każda komórka zajmie równą przestrzeń */
  min-height: 24px; /* Wysokość minimalna */
  /*border: 1px solid #ccc; */
  padding: 10px;
}
.cell-branches-label {
  flex: 7; /* Każda komórka zajmie równą przestrzeń */
  min-height: 24px; /* Wysokość minimalna */
  /*border: 1px solid #ccc; */
  padding: 10px;
  gap: 5px;
  display: flex;
}

:root {
  --bg-color: #E6D6C5;         /* Ciepły beż (tło główne, drewno) */
  --panel-color: #D1BFA5;       /* Jasny brąz (panele boczne, karty commitów) */
  --highlight-color: #B89B7E;   /* Ciemniejszy brąz (podświetlenia, aktywne elementy) */
  --text-color: #2E2E2E;        /* Grafitowy tekst (kontrast i czytelność) */
  --accent-green: #4C8C4A;      /* Zieleń (gałęzie, statusy, akcenty) */
  --commit-bg: #E0C9A6;         /* Jasny brązowy dla commitów */
  --commit-border: #8C6B48;     /* Ciemniejszy kontur commitów */
  --hover-color: #A3835F;       /* Subtelne podświetlenie na hover */
}

body {
  background-color: var(--bg-color);
  color: var(--text-color);
  font-family: 'Inter', sans-serif; /* Nowoczesny, czytelny font */
}

.table {
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
}

tr {
  display: flex;
  gap: 15px; /* Odstępy między commitami */
  position: relative;
}

th {
  position: relative;
  text-align: center;
}

.dot {
  width: 24px;
  height: 24px;
  /* border: 4px solid #4CAF50;  Obręcz zamiast wypełnienia */
  background-color: transparent; /* Puste wnętrze */
  /*border-radius: 50%; */
  position: relative;
  z-index: 2;
}

.dot-0 {
  width: 24px;
  height: 24px;
  border: 4px solid #4CAF50; /* Obręcz zamiast wypełnienia */
  background-color: transparent; /* Puste wnętrze */
  border-radius: 50%;
  position: relative;
  z-index: 2;
}

.dot-1 {
  width: 24px;
  height: 24px;
  border: 4px solid #4c63af; /* Obręcz zamiast wypełnienia */
  background-color: transparent; /* Puste wnętrze */
  border-radius: 50%;
  position: relative;
  z-index: 2;
}

.dot-2 {
  width: 24px;
  height: 24px;
  border: 4px solid #af4c84; /* Obręcz zamiast wypełnienia */
  background-color: transparent; /* Puste wnętrze */
  border-radius: 50%;
  position: relative;
  z-index: 2;
}

/* Linie poziome między kropkami */
th:not(:last-child)::after {
  content: "";
  position: absolute;
  width: 15px; /* Długość linii */
  height: 4px; /* Grubość linii */
  background-color: #4CAF50;
  top: 50%;
  left: 100%;
  transform: translateY(-50%);
}

.dot::before {
  content: "";
  position: absolute;
  width: 4px; /* Grubość linii */
  height: 5px; /* Długość linii (dopasuj do odstępów) */
  background-color: #4CAF50;
  top: -15px; /* Przesunięcie w górę, łączy poprzednią kropkę */
  left: 50%;
  transform: translateX(-50%);
  z-index: 1;
}

/* Usunięcie linii dla pierwszej kropki */
.dot:first-child::before {
  content: none;
}

.container {
  display: grid;
  grid-template-areas:
    "header header header"
    "branches content commit-details"
    "footer footer footer";
  grid-template-columns: 1fr 3fr 1fr;
}
.container > div {
  background-color: rgba(166, 63, 41, 0.08);
}

.container > div.branches {
  grid-area: branches;
}
.container > div.content {
  grid-area: content;
  width: auto;
  overflow: auto;
}
.container > div.footer {
  grid-area: footer;
}
.container > div.commit-details{
  grid-area: commit-details;
}

.branches {
  background: var(--commit-bg);
  border: 1px solid #4c63af;
  border-radius: 12px;
  padding: 10px;
  margin: 5px 0;
  transition: background 0.3s ease-in-out;
}

.branches-label-header {
  border: 1px solid var(--commit-border);
  background: white;
  display: flex;
  align-items: center; /* Wyrównanie pionowe */
  gap: 8px; /* Odstęp między GIF-em a napisem */
}

.branches-label {
  border: 1px solid var(--commit-border);
  display: flex;
  align-items: center; /* Wyrównanie pionowe */
  gap: 8px; /* Odstęp między GIF-em a napisem */
}

.branches-label-icon {
  width: 48px; /* Zmień na mniejszy/gif */
  height: 48px;
}

.commit-details {
  background: var(--commit-bg);
  border: 1px solid #4CAF50;
  border-radius: 12px;
  padding: 10px;
  margin: 5px 0;
  transition: background 0.3s ease-in-out;
}

li {
  list-style:none;
  padding: 2px 0 2px 0;
}

.content{
  max-height: 90vh;
  overflow-y: auto;

  background: var(--commit-bg);
  border: 1px solid var(--commit-border);
  border-radius: 12px;
  padding: 10px;
  margin: 5px 0;
  transition: background 0.3s ease-in-out;
}

.commit-hash {
  color: white;
  border-radius: 5px;
  background: rgba(16, 93, 194, 0.76);
  display:inline-block;
  width: 120px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: monospace;
}

.branch-label-0 {
  color: white;
  border-radius: 5px;
  background: rgba(16, 93, 194, 0.76);
  display:inline-block;
  width: auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: monospace;
  padding: 5px 5px;
}

.branch-label-1 {
  color: white;
  border-radius: 5px;
  background: rgba(194, 90, 16, 0.76);
  display:inline-block;
  width: auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: monospace;
  padding: 5px 5px;
}

.branch-label-2 {
  color: white;
  border-radius: 5px;
  background: rgb(76, 175, 80);
  display:inline-block;
  width: auto;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: monospace;
  padding: 5px 5px;
}

.commit-hash-label {
  --b: 0.1em;   /* the thickness of the line */
  color: white;
  border-radius: 5px;
  background: rgba(16, 93, 194, 0.76);
  display:inline-block;
  width: 120px;
  height: 24px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: monospace;

}

.commit-date-label {
  --b: 0.1em;   /* the thickness of the line */
  color: #000000;
  display:inline-block;
  height: 24px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: monospace;
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

.hover-3 {
  --b: 0.1em;   /* the thickness of the line */
  --c: rgba(38, 51, 66); /* the color */
  display:inline-block;
  color: rgb(255, 255, 255);
  padding-block: var(--b);
  background:
      linear-gradient(var(--c) 50%,#000 0) 0% calc(100% - var(--_p,0%))/100% 200%,
      linear-gradient(var(--c) 0 0) 0% var(--_p,0%)/var(--_p,0%) var(--b) no-repeat;
  -webkit-background-clip: text,padding-box;
  background-clip: text,padding-box;
  transition: .3s var(--_s,0s) linear,background-size .3s calc(.3s - var(--_s,0s));
  font-family: monospace;
}
.hover-3:hover {
  --_p: 100%;
  --_s: .3s;
}
.hover-3:hover .commit-hash {
  --_s: .12s;
  background: rgba(152, 41, 166);
  transition: .6s var(--_s,0s) linear,background-size .3s calc(.3s - var(--_s,0s));
}

.hover-3:hover .dot {
  --_s: .12s;
  background: #FF8C00;
  border: 4px solid #D2691E; /* Obręcz zamiast wypełnienia */
  transition: .6s var(--_s,0s) linear,background-size .3s calc(.3s - var(--_s,0s));
}

.description {
  padding: 0 0 0 0;
  opacity: 0;
  height: 0;
  overflow: hidden;
  visibility: hidden;
  transition: opacity 0.5s ease-in-out 0.5s, visibility 0s linear 0.5s;
}

.hover-3:hover .description {
  padding: 0 0 0 5px;
  color: rgb(4, 169, 109);
  opacity: 1;
  height: 0.5%;
  visibility: visible;
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