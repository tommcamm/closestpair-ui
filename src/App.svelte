<script lang="ts">
  import ButtonsPanel from './lib/ButtonsPanel.svelte';
import Graph from './lib/Graph.svelte';

let dots: { x: number; y: number }[] = [];
</script>

<main class="container">
  <p>
    Closest Pair UI concept
  </p>

  <Graph bind:dots={dots}/>
  <ButtonsPanel bind:dots={dots} />

</main>