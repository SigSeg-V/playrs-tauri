<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { emit, listen } from '@tauri-apps/api/event'

  // Sends event to rodio to play current queue
  async function playSound() {
    await invoke("play_sound")
  }
  
  // Sends event to rodio to pause current queue
  async function pauseSound() {
    await invoke("pause_sound")
  }

  // Sends event to rodio to pause current queue
  async function stopSound() {
    await invoke("stop_sound")
  }

  // Adds sound to rodio queue
  async function addToQueue(files: Array<string>) {
    if (files !== null) {
      await invoke("add_to_queue", {files: files})
    }
    else {
      await invoke("add_to_queue", {})
    }
  }

  async function openFiles() {
    await invoke("open_file_dialog")
  }

  async function openFolders() {
    await invoke("open_folder_dialog")
  }
  
  type FilePayload = { paths: string[] };

  // playlist as a string array do display to the gui
  $: playList = [];

  // gets the new playlist from the backend on update
  listen<Array<string>>("update-playlist", (event) => {
    playList = event.payload;
    console.log(event.payload)
  }).catch();

  (async () => {
    await listen<FilePayload>('open-files', (event) => {
      console.log(event.payload);
      addToQueue(event.payload.paths);
  })
})();


</script>

<div>
  <div class="row">
    <button on:click={playSound}>
      Play
    </button>

    <button on:click={pauseSound}>
      Pause
    </button>

    <button on:click={stopSound}>
      Stop
    </button>

    <button on:click={openFiles}>
      Open File Dialog
    </button>

    <button on:click={openFolders}>
      Open Folder Dialog
    </button>

  </div>
  {#each playList as rec}
  <li> {rec} </li>
  {/each}
</div>
