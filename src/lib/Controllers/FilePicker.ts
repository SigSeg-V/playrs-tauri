import { invoke } from "@tauri-apps/api/tauri"
import { emit, listen } from '@tauri-apps/api/event'
import type { Files } from "../Models/FilePicker";
import { addToQueue } from "./Playlist";

export async function openFiles() {
  const unlisten = await invoke("open_file_dialog");
  return unlisten;
}

export async function openFolders() {
  const unlisten = await invoke("open_folder_dialog");
  return unlisten;
}

export async function awaitFileDialog() {
  const unlisten = await listen<Files>('open-files', (event) => {
    console.log(event.payload);
    addToQueue(event.payload.paths);
  });
  return unlisten;
}