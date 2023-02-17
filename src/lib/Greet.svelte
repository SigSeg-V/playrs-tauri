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
  async function addToQueue(files?: Array<string>) {
    if (files !== null) {
      await invoke("add_to_queue", {files: files})
    }
  }

  let openDialog = false;

  async function openFolder() {
    await invoke("open_file_dialog")
  }

  
  type FilePayload = {files: string[] };

  (async () => {
    await listen<FilePayload>('open-files', (event) => {
      console.log(event.payload);
      addToQueue(event.payload.files);
  })
})();
  


  // emits the `click` event with the object payload
  emit('click', {
    theMessage: 'Tauri is awesome!',
  })

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

    <button on:click={openFolder}>
      Open Import Dialog
    </button>

    {#if openDialog}
      <p>
        openDialog
      </p>
    {/if}
    {#if !openDialog}
    <p>
      closeDialog
    </p>
    {/if}
  </div>
</div>
