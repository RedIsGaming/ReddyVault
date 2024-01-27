<script lang="ts">
  import type { Slogan } from "../../types";
  import { Link, Router } from "svelte-navigator";
  import { routes } from "../components/routing";

  export let responder: string = "";

  const slogan: Slogan = {
    description: "Reddyvault is an Open Source Password Manager written in Rust and Svelte.",
  };

  function scrollToNext(event: MouseEvent): void {
    event.preventDefault();
    const scroll: HTMLElement | null = document.getElementById("quality");

    scroll?.scrollIntoView({ 
      behavior: "smooth"
    });
  }
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
            <a href={route.url} target={route.target}>
              <button class="button button-uncta">
                {route.name}
              </button>
            </a>
          {/if}
        {/each}
      </Router>
    </article>
  </article>
</section>
