<script lang="ts">
  const date: number = new Date().getFullYear();
  const description: string = `Copyright © ReddyVault 2024 - ${date}. All Rights Reserved!`;
</script>

<footer class="bg-red-400 fixed bottom-0 left-0 w-full h-16 sm:h-12 border-black dark:border-white border-t-2">
  <section class="reddy-section m-auto h-full items-center">
    <article class="reddy-article p-0 text-center">
      <p class="text-dark dark:text-white text-lg">{description}</p>
    </article>
  </section>
</footer>
