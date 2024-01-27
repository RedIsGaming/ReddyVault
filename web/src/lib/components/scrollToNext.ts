import { writable, type Writable } from 'svelte/store';
import { onMount } from "svelte";

export const store: Writable<HTMLElement> = writable();

export function scrollToNext(): void {
  const scroll: HTMLElement = $store;

  scroll ? scroll.scrollIntoView({
    behavior: "smooth",
  }): null;
}

onMount((): void => {
  scrollToNext();
});
