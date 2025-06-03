<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import * as path from '@tauri-apps/api/path';

const dirPathVM = ref("C:\\Users\\muzud\\OneDrive\\ドキュメント\\temp");
const itemsVM = ref([]);

async function on_open_button_clicked() {
  console.log("［Open］ボタンを押したぜ。")
  // Open a dialog
  const dirPath = await open({
    multiple: false,
    directory: true,  // ディレクトリーを開く。
    defaultPath: dirPathVM.value
  });
  dirPathVM.value = dirPath
  itemsVM.value = await fetch_file_names();
}

async function on_refresh_button_clicked() {
  console.log("［Refresh］ボタンを押したぜ。")
  itemsVM.value = await fetch_file_names();
}

// Tauriのコマンドを呼び出し。
// 配列を返す。
async function fetch_file_names() {
  return await invoke('get_file_names', { dirPath: dirPathVM.value });
}
</script>

<template>
  <main class="container">
    <div class="row">
      <input style="width:80%; height: 10vh;" :value="dirPathVM">
      <button @click="on_open_button_clicked" style="width:20%; height: 10vh;">Open</button>
    </div>
    <div class="row">
      <button @click="on_refresh_button_clicked" style="width:100%; height: 10vh;">Refresh</button>
    </div>
    <select style="width:100%; height:80vh;" size="5">
      <option v-for="item in itemsVM" :key="item" :value="item">{{ item }}</option>
    </select>
  </main>
</template>

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
  height: 95vh;
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.row {
  display: flex;
  justify-content: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  padding: 0.6em 1.2em;
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

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
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