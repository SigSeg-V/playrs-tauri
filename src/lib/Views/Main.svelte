<script lang="ts">

  import { awaitFileDialog, openFiles, openFolders } from "../Controllers/FilePicker";
  import { getDuration, getPosition, pauseSound, playSound, printClockTime, stopSound } from "../Controllers/Player";
  import { getPlaylist, popPlaylist } from "../Controllers/Playlist";
  import type { ClockTime } from "../Models/Player";

  let plist: string[] = [];
  $: playlist = plist;
  async function updatePlaylist() {
    plist = await getPlaylist();
  }

  let duration: ClockTime;
  $: duration;
  async function updateDuration() {
    console.log("in update duration " + duration.secs);
    duration = await getDuration();
    console.log("finished updating duration");
  }

  // let pos: ClockTime;
  // $: position = printClockTime(pos);
  // async function updatePosition() {
  //   pos = await getPosition();
  // }

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

    <button on:click={popPlaylist}>
      Pop Playlist
    </button> 

  </div>
  
  {#await updateDuration()}
    no duration
  {:then _}
    DURATION:
    {printClockTime(duration)} / 
    {duration.secs}
  {/await} 

  <!-- POSITION:
  {position} / 
  {pos.secs}
  
  {#each playlist as rec}
    <li> {rec.split("/").at(-1)} </li>
  {/each} -->
   
</div>
