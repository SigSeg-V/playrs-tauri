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
  
  async function popPlayList() {
    await invoke("pop_playlist")
  }

  async function getRuntime() {

  }

  type FilePayload = { paths: string[] };

  type ClockTimePayload = {
    hours: number,
    mins: number, 
    secs: number,
    msecs: number,
  };

  // playlist as a string array do display to the gui
  let playlist: string[];
  $: playlist = [];

  // gets the new playlist from the backend on update
  async function getPlaylist() {
    await listen<Array<string>>("update-playlist", (event) => {
    playlist = event.payload;
    console.log(event.payload)
  }).catch(() => {
    console.log("Error occurred updating playlist");
  });
  }

  let duration: ClockTimePayload;
  $: duration;

  async function getDuration() {
    await listen<ClockTimePayload>("get-duration", (event) => {
      duration = event.payload;
      console.log(event.payload);
    }).catch(() => {
      console.log("Error occurred getting duration");
    });
  }



  


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

    <button on:click={popPlayList}>
      Pop Playlist
    </button>

  </div>

  {#await getDuration()}
    <p>
      Loading duration...
    </p>
  {:then _}
    Duration of current file: {duration.hours}h, {duration.mins}m, {duration.secs}s, {duration.msecs}ms
  {/await}

  {#await getPlaylist()}
    <p>
      Playlist Empty
    </p>
  {:then _}
    {#each playlist as rec}
      <li> {rec.split("/").at(-1)} </li>
    {/each}
  {/await}

</div>
