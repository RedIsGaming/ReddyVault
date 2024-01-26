<script lang="ts">
  import { onMount } from "svelte";
  import ky from "ky";
  import { localhost } from "../components/routing";
  import Intro from "../components/Intro.svelte";
  import Quality from "../components/Quality.svelte";

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

<Intro responder={responder} />
<Quality />
