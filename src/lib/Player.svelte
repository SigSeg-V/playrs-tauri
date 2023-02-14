<script>
    import AudioPlayer from './AudioPlayer.svelte'
  
    let audioTracks = [
      'file://Users/charlie/Documents/Rust/playrs/src-tauri/assets/music.mp3',
      'https://sveltejs.github.io/assets/music/strauss.mp3',
      'https://sveltejs.github.io/assets/music/holst.mp3',
      'https://sveltejs.github.io/assets/music/satie.mp3'
    ]
  </script>

  
  {#each audioTracks as src}
    <AudioPlayer {src} />
  {/each}