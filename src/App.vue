<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
import * as path from '@tauri-apps/api/path';

const filePathVM = ref("C:\\Users\\muzud\\OneDrive\\ドキュメント\\temp");
const textVM = ref("");

async function on_open_button_clicked() {
  console.log("［Open］ボタンを押したぜ。")
  // Open a dialog
  const filePath = await open({
    multiple: false,
    directory: false,
    defaultPath: filePathVM.value
  });
  filePathVM.value = filePath
}

async function on_save_button_clicked() {
  console.log("［Save］ボタンを押したぜ。")
  // 書き込むためには、📄 `src-tauri/capabilities/default.json` ファイルの `permissions` を設定する必要がある。
  await writeTextFile(filePathVM.value, textVM.value);
}

async function on_load_button_clicked() {
  console.log("［Load］ボタンを押したぜ。")
  const contents = await readTextFile(filePathVM.value);  
  textVM.value = contents
}
</script>

<template>
  <main class="container">
    <div class="row">
      <input style="width:80%; height: 10vh;" :value="filePathVM">
      <button @click="on_open_button_clicked" style="width:20%; height: 10vh;">Open</button>
    </div>
    <div class="row">
      <button @click="on_save_button_clicked" style="width:50%; height: 10vh;">Save</button>
      <button @click="on_load_button_clicked" style="width:50%; height: 10vh;">Load</button>
    </div>
    <textarea style="width:100%; height:80vh;" v-model="textVM"></textarea>
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