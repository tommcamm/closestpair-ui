<script lang="ts">
  import ButtonsPanel from './lib/ButtonsPanel.svelte';
import Graph from './lib/Graph.svelte';
import { onMount } from 'svelte';
  import { window } from '@tauri-apps/api';

let dots: { x: number; y: number }[] = [];
let closestPair: { x: number; y: number }[] = [];
let distance: number = 0;

// Fix the white screen problem, show the window only after is fully loaded
onMount(async () => {
    // This code runs after the component is added to the DOM.
    await window.getCurrent().show();
});

</script>
<main class="container">
  <div style="margin-top: -55px;">
    <h1>Closest Pair UI</h1>
    <p>
      <!-- show distance rounded with only two decimals -->
      Distance between points: <span style="color: red;">{distance.toFixed(2)}</span>
    </p>
  </div>


  <Graph bind:dots={dots} bind:closestPair={closestPair}/>
  <ButtonsPanel bind:dots={dots} bind:closestPair={closestPair} bind:distance={distance}/>

</main>