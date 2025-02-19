<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import {nextTick, ref} from "vue";
import { defineEmits } from "vue";
const emit = defineEmits(["close"]);

const repoUrl = ref("");
const destination = ref("");
const message_error = ref("");
const show_message_error = ref(false);

async function git_clone() {
  try {
    await invoke("git_clone", { repoPath: destination.value, url: repoUrl.value });
    show_message_error.value = false;
    await nextTick();
    emit("close");
  } catch (e) {
    console.error("Błąd:", e);
    message_error.value = typeof e === "string" ? e : JSON.stringify(e);
    show_message_error.value = true;
  }
}

async function selectFolder() {
  const selected = await open({ directory: true });
  if (selected) {
    destination.value = selected;
  }
}

</script>

<template>
  <h2 class="text-lg font-bold mb-4">Clone remote repository</h2>
  <input v-model="repoUrl" placeholder="Url to remote..." class="input mb-2" />
  <input v-model="destination" @click="selectFolder" placeholder="Clone to..." class="input"/>
  <button @click="$emit('close')" class="btn">Cancel</button>
  <button @click="git_clone"  class="btn">Clone</button>
  <div v-if="show_message_error"> {{ message_error}}</div>
</template>

<style scoped>
.input {
  border: 1px solid #ccc;
  width: 100%;
  padding: 0.5rem;
  margin: 0px 0px 10px 0px;
}
.btn {
  background-color: #4CAF50;
  color: white;
  border: none;
  margin: 10px;
  font-size: 16px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}
</style>