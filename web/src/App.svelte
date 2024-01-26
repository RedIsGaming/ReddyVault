<script lang="ts">
  import Navbar from "./lib/components/Navbar.svelte";
  import { Router, Route } from "svelte-navigator";
  import { routing } from "./lib/components/routing";
  import NotFound from "./lib/pages/NotFound.svelte";
  import Footer from "./lib/components/Footer.svelte";
</script>

<Navbar />
<main class="bg-white dark:bg-black">
  <Router>
    {#each routing as route}
      <Route path={route.url} component={route.component} />
      <Route path={"*"} component={NotFound} />
    {/each}
  </Router>
</main>
<!--<Footer />-->
