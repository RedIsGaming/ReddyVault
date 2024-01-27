<script lang="ts">
  import { Link, Router } from "svelte-navigator";
  import type { NotFound } from "../../types";
  import { Icon, ExclamationCircle } from "svelte-hero-icons";

  const notFound: NotFound = {
    name: "404 Page Not Found",
    description: "The page you are looking for couldn't be found!",
    return: "Return home"
  };
</script>

<section class="reddy-section custom-section">
  <article class="reddy-article">
    <div class="text-black dark:text-white inline-block space-x-0 sm:space-x-3 sm:flex">
      <h1 class="reddy-h1">{notFound.name}</h1>
      <Icon src={ExclamationCircle} size={"64"} />
    </div>
    <div class="text-xl md:text-2xl">
      <p class="mt-5 md:mt-10 text-black dark:text-white">
        {notFound.description}
        <Router>
          <Link to={"/"} class="text-red-500">
            {notFound.return}
          </Link>
        </Router>
      </p>
    </div>
  </article>
</section>
