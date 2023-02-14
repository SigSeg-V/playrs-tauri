<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function playSound() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("play_sound")
  }
  
  async function pauseSound() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("pause_sound")
  }

  async function addToQueue() {
    await invoke("add_to_queue")
  }

</script>

<div>
  <div class="row">
    <button on:click={playSound}>
      Play
    </button>
    <button on:click={pauseSound}>
      Pause
    </button>
    <button on:click={addToQueue}>
      Add To Queue
    </button>
  </div>
</div>
