import { invoke } from "@tauri-apps/api/tauri"
import { emit, listen } from '@tauri-apps/api/event'


// Adds sound to rodio queue
export async function addToQueue(files: string[]) {
  let unlisten;
  if (files !== null) {
    unlisten = await invoke("add_to_queue", {files: files});
  }
  else {
    unlisten = await invoke("add_to_queue", {});
  }
  return unlisten;
}

export async function popPlayList() {
    const unlisten = await invoke("pop_playlist");
    return unlisten;
}

// gets the new playlist from the backend on update
export async function getPlaylist(playlist: string[]) {
    const unlisten = await listen<Array<string>>("update-playlist", (event) => {
      playlist = event.payload;
      console.log(event.payload)
    });
    return unlisten;
}