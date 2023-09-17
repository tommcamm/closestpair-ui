<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { dialog } from "@tauri-apps/api";

  export let dots: { x: number; y: number }[] = [];
  export let closestPair: { x: number; y: number }[] = [];
  export let distance: number = 0;

  
  async function openCSVFileDialog() {
    try {
      const files = await dialog.open({
  multiple: false,
  filters: [{
    name: 'CSV Files',
    extensions: ['csv', 'CSV']
  }]});
      if (files && files.length) {
        const csvFilePath = files;
        console.log(csvFilePath);
        // call the tauri function read_csv_file and update the dots array
        const rawDots = await invoke("read_csv_file", { path: csvFilePath }) as number[][];
        dots = rawDots.map(([x, y]) => ({ x, y }));
        closestPair = [];
        distance = 0;
      }
    } catch (error) {
      console.error("Failed to open CSV file:", error);
    }
  }

  async function generateValues(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const rawDots = await invoke("get_dots") as number[][];
    dots = rawDots.map(([x, y]) => ({ x, y }));
    closestPair = [];
    distance = 0;
  }

  // Function to execute the closest pair algorithm on the backend
  async function executeAlgorithm(){
    const result = await invoke("get_closest_pair") as number[];
    closestPair[0] = { x: result[0], y: result[1] };
    closestPair[1] = { x: result[2], y: result[3] };
    distance = result[4];
  }
</script>

<div style="margin-top: 10px;">
  <div class="row">

    <!-- New button for manual dot addition from a CSV file -->
    <form class="row" on:submit|preventDefault={openCSVFileDialog}>
      <button type="submit">+</button>
    </form>

    <!-- Button to generate random dots -->
    <form style="margin-left: 10px;" on:submit|preventDefault={generateValues}>
      <button type="submit">Generate dots</button>
    </form>
    
    <!-- Button to execute the algorithm -->
    <form style="margin-left: 10px;" on:submit|preventDefault={executeAlgorithm}>
      <button type="submit">Execute Algorithm</button>
    </form>
  </div>
</div>
