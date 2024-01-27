<script lang="ts">
  import type { Slogan } from "../../types";
  import { Link, Router } from "svelte-navigator";
  import { routes } from "../components/routing";
  import { scrollToNext } from "./scrollToNext";

  export let responder: string = "";

  const slogan: Slogan = {
    description: "Reddyvault is an Open Source Password Manager written in Rust and Svelte.",
  };
</script>

<section class="reddy-section bg-red-100 dark:bg-red-900">
  <article class="reddy-article">
    <h1 class="reddy-h1">{responder}</h1>
    <p class="my-5 md:my-10 text-black dark:text-white text-xl md:text-2xl">{slogan.description}</p>

    <article class="grid auto-cols-auto sm:inline-block">
      <Router>
        {#each routes as route, index}
          {#if index == 0}
            <Link to={route.url}>
              <button class="button" on:click={scrollToNext}>{route.name}</button>
            </Link>
            {:else}
            <Link to={route.url} target={"_blank"}>
              <button class="button button-uncta">
                {route.name}
              </button>
            </Link>
          {/if}
        {/each}
      </Router>
    </article>
  </article>
</section>
