<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { listen } from '@tauri-apps/api/event'

  // Sends event to rodio to play current queue
  async function playSound() {
    const unlisten = await invoke("play_sound");
    return unlisten;
  }
  
  // Sends event to rodio to pause current queue
  async function pauseSound() {
    const unlisten = await invoke("pause_sound");
    return unlisten;
  }

  // Sends event to rodio to pause current queue
  async function stopSound() {
    const unlisten = await invoke("stop_sound");
    return unlisten;
  }

  // Adds sound to rodio queue
  async function addToQueue(files: Array<string>) {
    let unlisten;
    if (files !== null) {
      unlisten = await invoke("add_to_queue", {files: files});
    }
    else {
      unlisten = await invoke("add_to_queue", {});
    }
    return unlisten;
  }

  async function openFiles() {
    const unlisten = await invoke("open_file_dialog");
    return unlisten;
  }

  async function openFolders() {
    const unlisten = await invoke("open_folder_dialog");
    return unlisten;
  }
  
  async function popPlayList() {
    const unlisten = await invoke("pop_playlist");
    return unlisten;
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
    const unlisten = await listen<Array<string>>("update-playlist", (event) => {
      playlist = event.payload;
      console.log(event.payload)
    });
    return unlisten;
  }

  invoke("get_duration");

  let duration: ClockTimePayload;
  $: duration = null;

  async function getDuration() {
    console.log("duration button clicked");

    const unlisten = await listen<ClockTimePayload>("get-duration", (event) => {
      
      // duration.hours = event.payload.hours;
      // duration.mins = event.payload.mins;
      // duration.secs = event.payload.secs;
      // duration.msecs = event.payload.msecs;
      console.log("inside await");
      console.log(event.payload);
    });
    return unlisten;
  }

  (async () => {
    const unlisten = await listen<FilePayload>('open-files', (event) => {
      console.log(event.payload);
      addToQueue(event.payload.paths);
    });
    return unlisten;
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

    <button on:click={getDuration}>
      Get Duration
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
