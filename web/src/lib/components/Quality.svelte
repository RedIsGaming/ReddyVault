<script lang="ts">
  import type { Quality } from "../../types";
  import { Icon, CheckCircle, Square3Stack3d, ServerStack } from "svelte-hero-icons";
  
  const qualities: Quality[] = [
    {
      name: "Why Reddy's Vault?",
      description: "",
      icon: ""
    },
    {
      name: "Easy to use",
      description: "Simple UI and easy language makes it great to find the content you are looking for. Minimal design is involved.",
      icon: CheckCircle
    },
    {
      name: "Blazingly fast",
      description: "Modern languages and frameworks requires modern solutions. ReddyVault is using Rust/Svelte for great performance.",
      icon: ServerStack
    },
    {
      name: "No gotchas",
      description: "No hidden URLs or content. Everything you see is where you can navigate through. Everything is clean and concise.",
      icon: Square3Stack3d
    },
  ];
</script>

<section class="reddy-section m-auto">
  <article class="reddy-article">
    <h2 class="reddy-h2 text-black dark:text-white">{qualities.map(quality => quality.name).shift()}</h2>
    
    <article class="my-5 md:my-10 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
      {#each qualities.slice(1) as quality}
        <div class="border-gray-200 dark:border-gray-800 border-2 p-5 rounded-3xl text-center">
          <figure class="text-red-500 flex justify-center">
            <Icon src={quality.icon} size={"64"} />
          </figure>
          <h3 class="text-3xl md:text-4xl text-black dark:text-white my-5">{quality.name}</h3>
          <p class="text-black text-base md:text-lg dark:text-white">{quality.description}</p>
        </div>
      {/each}
    </article>
  </article>
</section>
