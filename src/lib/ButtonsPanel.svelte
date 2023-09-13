<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let dots = [];

  async function generateValues(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    dots = await invoke("get_dots") as { x: number; y: number }[];
    console.log(dots);
  }
</script>

<div style="margin-top: 10px;">
  <div class="row">
    <form class="row" on:submit|preventDefault={generateValues}>
      <button type="submit">Generate dots</button>
    </form>
    <form style="margin-left: 10px;">
      <button>Execute Algorithm</button>
    </form>
  </div>
</div>