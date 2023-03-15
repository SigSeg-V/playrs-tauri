<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { emit, listen } from '@tauri-apps/api/event'

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

  type ClockTime = {
    hours: number,
    mins: number, 
    secs: number,
    msecs: number,
  };

  function printClockTime(ct: ClockTime) {
    let secs: number = ct.secs;
    let mins: number = ct.secs / 60;
    let hours: number = mins / 60;

    secs %= 60;
    mins %= 60;
    
    secs = Math.round(secs);
    mins = Math.floor(mins);
    hours = Math.floor(hours);

    let time = [hours, mins, secs];
    let timeArray: string[] = [];
    let timeString = "";

    time.forEach(denom => {
      let formattedNum = denom.toLocaleString('en-US', {
        minimumIntegerDigits: 2,
        useGrouping: false
      });
      timeArray.push(formattedNum);
    });
    return timeString.concat(timeArray[0], ":", timeArray[1], ":", timeArray[2]);
  }

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

  let duration: ClockTime;
  $: duration = {hours: 0, mins: 0, secs: 0, msecs: 0};

  async function getDuration() {
    console.log("duration button clicked");

    const unlisten = await listen<ClockTime>("get-duration", (event) => {
      
      duration = event.payload;
      console.log("inside await get-duration");
      console.log(event.payload);
    });
    return unlisten;
  }

  let position: ClockTime;
  $: position = {hours: 0, mins: 0, secs: 0, msecs: 0};

  async function getPosition() {
    console.log("position button clicked");

    const unlisten = await listen<ClockTime>("get-position", (event) => {
      
      position = event.payload;
      console.log("inside await get-position");
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
    DURATION:
    {printClockTime(duration)} / 
    {duration.secs}
  {/await}
  
  {#await getPosition()}
    <p>
      Loading position...
    </p>
  {:then _}
    POSITION:
    {printClockTime(position)} / 
    {position.secs}
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
