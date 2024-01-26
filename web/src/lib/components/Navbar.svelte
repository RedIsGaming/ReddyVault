<script lang="ts">
  import vite from "/vite.svg";
  import { Link, Router } from "svelte-navigator";
  import { Icon, Bars3 } from "svelte-hero-icons";
  import { routing } from "./routing";

  export const reddyVault: string = "ReddyVault";
</script>

<header class="fixed inset-0">
  <nav class="h-16 flex justify-center">
    <ul class="flex items-center w-full lg:w-2/3 mx-3 lg:mx-0">
      <Router>
        <li class="w-full flex justify-start items-center">
          <Link to="/">
            <figure class="flex">
              <img src={vite} alt="Vite" />
              <figcaption class="text-black dark:text-white text-2xl ml-2 font-semibold">{reddyVault}</figcaption>
            </figure>
          </Link>
        </li>
        <li class="w-full hidden justify-end items-center text-lg sm:flex">
          {#each routing.slice(1) as item, index}
            {#if index == routing.length - 2}
              <Link to={item.url} class="ml-20 text-white">
                <button class="bg-red-600 py-2 px-4 rounded-xl hover:bg-red-500 duration-300 ease-in-out">{item.name}</button>
              </Link>
              {:else}
              <Link to={item.url} class="ml-20 text-black dark:text-white">
                {item.name}
              </Link>
            {/if}
          {/each}
        </li>
        <li class="w-full flex justify-end items-center sm:hidden text-black dark:text-white">
          <Icon src={Bars3} size={"48"} />
        </li>
      </Router>
    </ul>
  </nav>
</header>
