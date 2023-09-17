<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  export let dots: { x: number; y: number }[] = [];
  export let closestPair: { x: number; y: number }[] = [];

  async function generateValues(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const rawDots = await invoke("get_dots") as number[][];
    dots = rawDots.map(([x, y]) => ({ x, y }));
    closestPair = [];
  }

  // Function to execute the closest pair algorithm on the backend
  async function executeAlgorithm(){
    const result = await invoke("get_closest_pair") as number[];
    closestPair[0] = { x: result[0], y: result[1] };
    closestPair[1] = { x: result[2], y: result[3] };
  }
</script>

<div style="margin-top: 10px;">
  <div class="row">
    <form class="row" on:submit|preventDefault={generateValues}>
      <button type="submit">Generate dots</button>
    </form>
    <form style="margin-left: 10px;" on:submit|preventDefault={executeAlgorithm}>
      <button type="submit">Execute Algorithm</button>
    </form>
  </div>
</div>