<script>
let { children } = $props()

const title = 'Ephemeride'
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<div class="app-wrapper">
  {@render children()}
</div>
