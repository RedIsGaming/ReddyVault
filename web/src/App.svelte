<script lang="ts">
  import Navbar from "./lib/components/Navbar.svelte";
  import { Router, Route } from "svelte-navigator";
  import type { Routing } from "./types";
  import Home from "./lib/pages/Home.svelte";
  import About from "./lib/pages/About.svelte";
  import Login from "./lib/pages/Login.svelte";

  const routing: Routing[] = [
    {
      url: "/",
      component: Home
    },
    {
      url: "/about",
      component: About
    },
    {
      url: "/login",
      component: Login
    },
  ];
</script>

<main>
  <Router>
    <Navbar />
    {#each routing as route}
      <Route path={route.url} component={route.component} />
    {/each}
  </Router>
</main>
