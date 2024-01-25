<script lang="ts">
  import vite from "/vite.svg";
  import { Link, Router } from "svelte-navigator";
  import type { ListItem } from "../types";

  const listItem: ListItem[] = [
    {
      url: "/",
      name: "url1"
    },
    {
      url: "/",
      name: "url2"
    },
    {
      url: "/",
      name: "url3"
    },
    {
      url: "/",
      name: "Login"
    }
  ];
</script>

<header class="fixed inset-0">
  <nav class="h-16 flex justify-center">
    <ul class="flex items-center w-full lg:w-2/3 mx-3 lg:mx-0">
      <Router>
        <li class="w-full flex justify-start items-center">
          <Link to="/">
            <figure class="flex">
              <img src={vite} alt="Vite" />
              <figcaption class="text-black dark:text-white text-lg ml-2">ReddyVault</figcaption>
            </figure>
          </Link>
        </li>
        <li class="w-full hidden justify-end items-center text-lg sm:flex">
          {#each listItem as item, index}
            {#if index == listItem.length - 1}
              <Link to={item.url} class="ml-20 text-white">
                <button class="bg-red-500 py-2 px-4 rounded-xl hover:bg-red-600 duration-300 ease-in-out">{item.name}</button>
              </Link>
              {:else}
              <Link to={item.url} class="ml-20 text-black dark:text-white">
                {item.name}
              </Link>
            {/if}
          {/each}
        </li>
      </Router>
    </ul>
  </nav>
</header>
