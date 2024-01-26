<script lang="ts">
  import { onMount } from "svelte";
  import ky from "ky";
  import { localhost } from "../components/routing";

  let responder: string = "";

  onMount(async (): Promise<void> => {
    try {
      responder = await ky.get(localhost)
        .then((resolve: Response) => resolve.text())
        .catch((reject: Error) => reject.message);
    }
    
    catch (error: unknown) {
      console.error("Failed connecting to Reddy's Vault!", error);
    }
  });
</script>

<section class="mt-16 flex justify-center">
  <article class="w-full py-12 lg:w-2/3 mx-3 lg:mx-0">
    <h1 class="text-4xl text-black dark:text-white">{responder}</h1>
  </article>
</section>
