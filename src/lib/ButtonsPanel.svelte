<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  export let dots: { x: number; y: number }[] = [];

  async function generateValues(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const rawDots = await invoke("get_dots") as number[][];
    dots = rawDots.map(([x, y]) => ({ x, y }));
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