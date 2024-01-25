<script lang="ts">
  import { onMount } from "svelte";
  import ky from "ky";

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

<section>
  <h1 class="text-3xl">{response}</h1>
</section>
