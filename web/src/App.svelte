<script lang="ts">
  import Navbar from "./lib/components/Navbar.svelte";
  import { Router, Route } from "svelte-navigator";
  import { routing } from "./lib/components/routing";
</script>

<Navbar />
<main>
  <Router>
    {#each routing as route}
      <Route path={route.url} component={route.component} />
    {/each}
  </Router>
</main>
