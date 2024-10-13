<template>
  <div class="editor-container">
    <textarea :style="{ fontSize: `${fontSize}pt` }" v-model="contents" />
  </div>
  <v-dialog v-model="isSaveDialogOpen" max-width="500">
    <v-card>
      <v-card-title>Do you want to save the changes you made ?</v-card-title>
      <v-card-text>Your changes will be lost if you don't save them</v-card-text>

      <template #actions>
        <v-btn @click="saveDialogCallback?.()">Don't Save</v-btn>
        <v-btn @click="isSaveDialogOpen = false">Cancel</v-btn>
        <v-btn @click="saveAndThen(saveDialogCallback)">Save</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { once } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";

const DEFAULT_FONT_SIZE = 10;
const fontSize = ref(DEFAULT_FONT_SIZE);
const fileName = ref("");
const latestContents = ref("");
const contents = ref("");
const isSaveDialogOpen = ref(false);
const saveDialogCallback = ref<() => any>();
const tauriWindow = getCurrentWindow();

watch(fileName, (newFileName) => {
  tauriWindow.setTitle(`${newFileName} - NTR Pad`);
});

onTauriEvent("invoke:new_file", () => askSaveChangesAndThen(clear));
onTauriEvent("invoke:open_file", () => askSaveChangesAndThen(openFile));
onTauriEvent("invoke:save", save);
onTauriEvent("invoke:save_as", () => invoke("save_as", { contents: contents.value }));
onTauriEvent("invoke:undo", () => {
  // !TODO: implement me
});
onTauriEvent("invoke:redo", () => {
  // !TODO: implement me
});

onTauriEvent<string>("update-file-name", ({ payload }) => {
  fileName.value = payload;
});
onTauriEvent<string>("file-open", ({ payload }) => {
  latestContents.value = payload;
  contents.value = payload;
});

onTauriEvent<string>("zoom-in", () => fontSize.value++);
onTauriEvent<string>("zoom-out", () => fontSize.value--);
onTauriEvent<string>("reset-zoom", () => (fontSize.value = DEFAULT_FONT_SIZE));

async function clear(): Promise<void> {
  fileName.value = "New";
  contents.value = "";
  await invoke("new_file", { contents: contents.value });
}

async function openFile(): Promise<void> {
  await invoke("open_file");
}

function askSaveChangesAndThen(cb: () => any): void {
  if (latestContents.value !== contents.value) {
    saveDialogCallback.value = () => {
      cb();
      isSaveDialogOpen.value = false;
      saveDialogCallback.value = undefined;
    };
    isSaveDialogOpen.value = true;
  } else {
    cb();
  }
}

async function saveAndThen(cb: (() => any) | undefined): Promise<void> {
  await save();
  const unlisten = await once("file-saved", () => cb?.());
  once("file-saving-canceled", unlisten);
}

function save(): Promise<unknown> {
  return invoke("save", { contents: contents.value });
}

clear();
</script>

<style scoped lang="scss">
.editor-container {
  width: 100%;
  height: 100%;
  overflow: hidden;

  textarea {
    width: 100%;
    height: 100%;
    padding: 8px;
    resize: none;
  }
}
</style>
