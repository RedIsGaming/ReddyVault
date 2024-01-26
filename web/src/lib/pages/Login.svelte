<script lang="ts">
  import { onMount } from "svelte";
  import ky from "ky";
  import { localhost } from "../components/routing";

  let responder: string = "";

  onMount(async (): Promise<void> => {
    try {
      responder = await ky.get(localhost.concat("login"))
        .then((resolve: Response) => resolve.text())
        .catch((reject: Error) => reject.message);
    }
    
    catch (error: unknown) {
      console.error("Failed connecting to Reddy's Vault!", error);
    }
  });
</script>

<section class="reddy-section">
  <article class="reddy-article">
    <h1 class="reddy-h1">{responder}</h1>
  </article>
</section>
