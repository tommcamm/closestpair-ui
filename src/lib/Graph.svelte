<script>

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

export const data = {
    datasets: [{
    label: 'L1',
    data: [{
      x: -10,
      y: 0
    }, {
      x: 0,
      y: 10
    }, {
      x: 10,
      y: 5
    }, {
      x: 0.5,
      y: 5.5
    },
    {
      x: 5,
      y: 0
    }, {
      x: 3,
      y: -5
    }, {
      x: 1,
      y: 9
    }, {
      x: 10,
      y: 3
    }],
    backgroundColor: 'rgb(255, 99, 132)'
  }],
}
</script>


<Scatter { data} options={{ responsive:  true }} />