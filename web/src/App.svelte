<script lang="ts">
  import { onMount } from "svelte";
  import ky from "ky";
  import Navbar from "./lib/Navbar.svelte";

  let response: unknown;
  const localhost: string = "http://localhost:8080/";

  onMount(async (): Promise<void> => {
    try {
      response = await ky.get(localhost)
        .then((resolve: Response) => localhost.endsWith("8080/") ? resolve.text() : "Something broke on Reddy's Vault!")
        .catch((reject: Error) => reject.message);
    } 
    
    catch (error: unknown) {
      console.error("Failed connecting to Reddy's Vault!", error);
    }
  });
</script>

<Navbar />
<main>
  <section class="mt-16 flex justify-center">
    <article class="w-full py-12 lg:w-2/3 mx-3 lg:mx-0">
      <h1 class="text-4xl text-black dark:text-white">{response}</h1>
    </article>
  </section>
</main>
