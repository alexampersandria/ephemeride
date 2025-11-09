<script>
  import Heatmap from '$lib/components/Heatmap.svelte'
  import DocsExample from '$lib/components/utils/DocsExample.svelte'

  let data = [
    { date: '2023-01-01', value: 3 },
    { date: '2023-01-02', value: 5 },
    { date: '2023-01-03', value: 2 },
    { date: '2023-01-04', value: 4 },
    { date: '2023-01-05', value: 1 },
    // ...more data points
    { date: '2023-12-28', value: 2 },
    { date: '2023-12-29', value: 5 },
    { date: '2023-12-30', value: 3 },
    { date: '2023-12-31', value: 4 }
  ]
</script>

# Heatmap

<DocsExample left>
  <Heatmap {data} year={2023} />
</DocsExample>
