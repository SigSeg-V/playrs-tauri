<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { emit, listen } from '@tauri-apps/api/event'
  import { nullDuration, type ClockTime } from "../Models/Player";
  import { awaitFileDialog, openFiles, openFolders } from "../Controllers/FilePicker";
  import { getDuration, getPosition, pauseSound, playSound, printClockTime, stopSound } from "../Controllers/Player";
  import { getPlaylist, popPlayList } from "../Controllers/Playlist";

  let playlist: string[];
  $: playlist = [];

  let duration: ClockTime;
  $: duration = nullDuration;

  let position: ClockTime;
  $: position = nullDuration;

</script>

<!--
  using await here to get file dialog, nicer than the shit going on before
-->
<!-- svelte-ignore empty-block -->
{#await awaitFileDialog()}
{/await}

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

  {#await getDuration(duration)}
    <p>
      Loading duration...
    </p>
  {:then _}
    DURATION:
    {printClockTime(duration)} / 
    {duration.secs}
  {/await}
  
  {#await getPosition(position)}
    <p>
      Loading position...
    </p>
  {:then _}
    POSITION:
    {printClockTime(position)} / 
    {position.secs}
  {/await}
    

  {#await getPlaylist(playlist)}
    <p>
      Playlist Empty
    </p>
  {:then _}
    {#each playlist as rec}
      <li> {rec.split("/").at(-1)} </li>
    {/each}
  {/await}

</div>
