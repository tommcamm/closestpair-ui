<script lang="ts">

import { Scatter } from 'svelte-chartjs'; 

import { 
    Chart as ChartJS,
    Title,
    Tooltip,
    Legend, 
    LineElement, 
    CategoryScale, 
    LinearScale, 
    PointElement
} from 'chart.js';

ChartJS.register(
    Title,
    Tooltip,
    Legend,
    LineElement,
    CategoryScale,
    LinearScale,
    PointElement
);

export let dots: { x: number; y: number }[] = [];

$: formattedDots = dots.map(dot => ({ x: dot.x, y: dot.y }));
$: data = {
    datasets: [{
        label: 'Generated Dots',
        data: formattedDots,
        backgroundColor: 'rgb(255, 99, 132)'
    }],
};

</script>

<Scatter { data } options={{ responsive:  true }} />